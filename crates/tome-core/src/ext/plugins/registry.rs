//! Plugin registry - manages loaded plugins and their registrations.

use std::collections::HashMap;
use std::path::PathBuf;

use extism::convert::Json;
use extism::UserData;

pub use crate::ext::plugins::types::{ActionInput, ActionOutput, CommandInput, EditorState, HookInput};


use crate::selection::Selection;

use linkme::distributed_slice;

/// Function type for creating host functions.

pub type HostFunctionFactory = fn(SharedHostContext) -> Vec<extism::Function>;

/// Registry for host function factories (compile-time collection).
#[distributed_slice]
pub static HOST_FUNCTION_FACTORIES: [HostFunctionFactory];


/// Context passed to host functions, providing access to editor state.
pub struct PluginHostContext {
    /// Document text (copy for safety - plugins can't corrupt the real buffer).
    pub text: String,
    /// Current selection (primary range only for simplicity).
    pub selection_anchor: usize,
    pub selection_head: usize,
    /// Cursor position.
    pub cursor: usize,
    /// Pending operations to apply after plugin call.
    pub pending_ops: Vec<PendingOp>,
    /// Messages to display.
    pub messages: Vec<String>,
    /// Configuration values (key -> json string).
    pub config: HashMap<String, String>,
}

/// Operations queued by plugins to be applied after the call completes.

#[derive(Debug, Clone)]
pub enum PendingOp {
    SetCursor(usize),
    SetSelection { anchor: usize, head: usize },
    Insert(String),
    Delete,
    Message(String),
    OpenFile(String),
}


impl PluginHostContext {
    pub fn new(text: String, selection: &Selection, cursor: usize) -> Self {
        let primary = selection.primary();
        Self {
            text,
            selection_anchor: primary.anchor,
            selection_head: primary.head,
            cursor,
            pending_ops: Vec::new(),
            messages: Vec::new(),
            config: HashMap::new(),
        }
    }

    pub fn update(&mut self, text: String, selection: &Selection, cursor: usize, config: HashMap<String, String>) {
        let primary = selection.primary();
        self.text = text;
        self.selection_anchor = primary.anchor;
        self.selection_head = primary.head;
        self.cursor = cursor;
        self.pending_ops.clear();
        self.messages.clear();
        self.config = config;
    }



    pub fn take_pending_ops(&mut self) -> Vec<PendingOp> {
        std::mem::take(&mut self.pending_ops)
    }

    pub fn take_messages(&mut self) -> Vec<String> {
        std::mem::take(&mut self.messages)
    }
}

/// Wrapper type for sharing host context with extism.
/// UserData<T> internally wraps T in Arc<Mutex<T>>.
pub type SharedHostContext = UserData<PluginHostContext>;

/// Parameters for calling a plugin action.
pub struct PluginActionParams<'a> {
    pub name: &'a str,
    pub count: usize,
    pub extend: bool,
    pub char_arg: Option<char>,
}

/// Context for plugin execution.
pub struct PluginContext<'a> {
    pub text: &'a str,
    pub selection: &'a Selection,
    pub cursor: usize,
    pub config: HashMap<String, String>,
}

/// A loaded plugin with its metadata and the actual extism Plugin instance.

pub struct LoadedPlugin {
    /// Unique plugin ID.
    pub id: String,
    /// Plugin display name.
    pub name: String,
    /// Plugin version.
    pub version: String,
    /// Path to the plugin file.
    pub path: PathBuf,
    /// Actions registered by this plugin.
    pub actions: Vec<String>,
    /// Commands registered by this plugin.
    pub commands: Vec<String>,
    /// Hooks this plugin subscribes to.
    pub hooks: Vec<String>,
    /// Keybindings registered by this plugin.
    pub keybindings: Vec<PluginKeybinding>,
    /// The actual extism plugin instance.
    plugin: extism::Plugin,
    /// Shared host context for this plugin.
    host_ctx: SharedHostContext,
}

impl LoadedPlugin {
    pub fn new(
        id: String,
        name: String,
        version: String,
        path: PathBuf,
        registration: PluginRegistration,
        plugin: extism::Plugin,
        host_ctx: SharedHostContext,
    ) -> Self {
        Self {
            id,
            name,
            version,
            path,
            actions: registration.actions.iter().map(|a| a.name.clone()).collect(),
            commands: registration.commands.iter().map(|c| c.name.clone()).collect(),
            hooks: registration.hooks,
            keybindings: registration.keybindings,
            plugin,
            host_ctx,
        }
    }

    /// Call an action on this plugin.
    pub fn call_action(
        &mut self,
        params: PluginActionParams,
        context: PluginContext,
    ) -> Result<ActionOutput, PluginCallError> {
        let primary = context.selection.primary();
        
        // Update host context with current editor state
        {
            let inner = self.host_ctx.get().map_err(|e| PluginCallError::LockError(e.to_string()))?;
            let mut ctx = inner.lock().map_err(|e| PluginCallError::LockError(e.to_string()))?;
            ctx.update(context.text.to_string(), context.selection, context.cursor, context.config);
        }


        // Build input
        let input = ActionInput {
            action_name: params.name.to_string(),
            count: params.count,
            extend: params.extend,
            char_arg: params.char_arg,
            editor: EditorState {
                text: context.text.to_string(),
                cursor: context.cursor,
                selection_anchor: primary.anchor,
                selection_head: primary.head,
            },
        };

        // Call the plugin's on_action export
        let Json(output): Json<ActionOutput> = self.plugin
            .call("on_action", Json(input))
            .map_err(|e| PluginCallError::CallFailed(e.to_string()))?;

        // Merge any pending ops from host function calls into the output
        let inner = self.host_ctx.get().map_err(|e| PluginCallError::LockError(e.to_string()))?;
        let ctx = inner.lock().map_err(|e| PluginCallError::LockError(e.to_string()))?;
        let mut final_output = output;
        
        for op in &ctx.pending_ops {
            match op {
                PendingOp::SetCursor(pos) => final_output.set_cursor = Some(*pos),
                PendingOp::SetSelection { anchor, head } => final_output.set_selection = Some((*anchor, *head)),
                PendingOp::Insert(text) => final_output.insert_text = Some(text.clone()),
                PendingOp::Delete => final_output.delete = true,
                PendingOp::Message(msg) => final_output.message = Some(msg.clone()),
                PendingOp::OpenFile(path) => final_output.open_file = Some(path.clone()),
            }
        }


        Ok(final_output)
    }

    /// Call a command on this plugin.
    pub fn call_command(
        &mut self,
        command_name: &str,
        args: Vec<String>,
        context: PluginContext,
    ) -> Result<ActionOutput, PluginCallError> {
        let primary = context.selection.primary();
        
        // Update host context
        {
            let inner = self.host_ctx.get().map_err(|e| PluginCallError::LockError(e.to_string()))?;
            let mut ctx = inner.lock().map_err(|e| PluginCallError::LockError(e.to_string()))?;
            ctx.update(context.text.to_string(), context.selection, context.cursor, context.config);
        }


        let input = CommandInput {
            command_name: command_name.to_string(),
            args,
            editor: EditorState {
                text: context.text.to_string(),
                cursor: context.cursor,
                selection_anchor: primary.anchor,
                selection_head: primary.head,
            },
        };

        let Json(output): Json<ActionOutput> = self.plugin
            .call("on_command", Json(input))
            .map_err(|e| PluginCallError::CallFailed(e.to_string()))?;

        Ok(output)
    }

    /// Call a hook on this plugin.
    pub fn call_hook(
        &mut self,
        hook_name: &str,
        extra: serde_json::Value,
        context: PluginContext,
    ) -> Result<(), PluginCallError> {
        let primary = context.selection.primary();
        
        // Update host context
        {
            let inner = self.host_ctx.get().map_err(|e| PluginCallError::LockError(e.to_string()))?;
            let mut ctx = inner.lock().map_err(|e| PluginCallError::LockError(e.to_string()))?;
            ctx.update(context.text.to_string(), context.selection, context.cursor, context.config);
        }


        let input = HookInput {
            hook_name: hook_name.to_string(),
            editor: EditorState {
                text: context.text.to_string(),
                cursor: context.cursor,
                selection_anchor: primary.anchor,
                selection_head: primary.head,
            },
            extra,
        };

        // Hooks don't return anything meaningful
        let _: Json<serde_json::Value> = self.plugin
            .call("on_hook", Json(input))
            .map_err(|e| PluginCallError::CallFailed(e.to_string()))?;

        Ok(())
    }
}

/// Error during plugin call.
#[derive(Debug, Clone)]
pub enum PluginCallError {
    LockError(String),
    CallFailed(String),
}

impl std::fmt::Display for PluginCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginCallError::LockError(e) => write!(f, "lock error: {}", e),
            PluginCallError::CallFailed(e) => write!(f, "plugin call failed: {}", e),
        }
    }
}

impl std::error::Error for PluginCallError {}

/// A keybinding registered by a plugin.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PluginKeybinding {
    pub mode: String,
    pub key: String,
    pub action: String,
}

/// JSON schema returned by plugin's `plugin_init` export.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PluginRegistration {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub actions: Vec<ActionRegistration>,
    #[serde(default)]
    pub commands: Vec<CommandRegistration>,
    #[serde(default)]
    pub hooks: Vec<String>,
    #[serde(default)]
    pub keybindings: Vec<PluginKeybinding>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionRegistration {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommandRegistration {
    pub name: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub description: String,
}

/// Registry of all loaded plugins.
pub struct PluginRegistry {
    /// Plugins by ID.
    plugins: HashMap<String, LoadedPlugin>,
    /// Action name -> plugin ID mapping.
    action_to_plugin: HashMap<String, String>,
    /// Command name -> plugin ID mapping.
    command_to_plugin: HashMap<String, String>,
    /// Hook name -> list of plugin IDs.
    hook_subscribers: HashMap<String, Vec<String>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            action_to_plugin: HashMap::new(),
            command_to_plugin: HashMap::new(),
            hook_subscribers: HashMap::new(),
        }
    }

    /// Register a plugin after loading.
    pub fn register(&mut self, plugin: LoadedPlugin) {
        let id = plugin.id.clone();

        // Map actions to plugin
        for action in &plugin.actions {
            self.action_to_plugin.insert(action.clone(), id.clone());
        }

        // Map commands to plugin
        for command in &plugin.commands {
            self.command_to_plugin.insert(command.clone(), id.clone());
        }

        // Register hook subscriptions
        for hook in &plugin.hooks {
            self.hook_subscribers
                .entry(hook.clone())
                .or_default()
                .push(id.clone());
        }

        self.plugins.insert(id, plugin);
    }

    /// Unload a plugin by ID.
    pub fn unload(&mut self, id: &str) -> Option<LoadedPlugin> {
        if let Some(plugin) = self.plugins.remove(id) {
            // Remove action mappings
            for action in &plugin.actions {
                self.action_to_plugin.remove(action);
            }
            // Remove command mappings
            for command in &plugin.commands {
                self.command_to_plugin.remove(command);
            }
            // Remove hook subscriptions
            for hook in &plugin.hooks {
                if let Some(subscribers) = self.hook_subscribers.get_mut(hook) {
                    subscribers.retain(|s| s != id);
                }
            }
            Some(plugin)
        } else {
            None
        }
    }

    /// Find which plugin handles an action.
    pub fn find_action_plugin(&self, action: &str) -> Option<&str> {
        self.action_to_plugin.get(action).map(|s| s.as_str())
    }

    /// Find which plugin handles a command.
    pub fn find_command_plugin(&self, command: &str) -> Option<&str> {
        self.command_to_plugin.get(command).map(|s| s.as_str())
    }

    /// Get plugins subscribed to a hook.
    pub fn get_hook_subscribers(&self, hook: &str) -> Vec<&str> {
        self.hook_subscribers
            .get(hook)
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Get a plugin by ID.
    pub fn get(&self, id: &str) -> Option<&LoadedPlugin> {
        self.plugins.get(id)
    }

    /// Get a mutable reference to a plugin by ID.
    pub fn get_mut(&mut self, id: &str) -> Option<&mut LoadedPlugin> {
        self.plugins.get_mut(id)
    }

    /// List all loaded plugins.
    pub fn list(&self) -> impl Iterator<Item = &LoadedPlugin> {
        self.plugins.values()
    }

    /// Execute an action if a plugin provides it.
    /// Returns None if no plugin handles this action.
    pub fn execute_action(
        &mut self,
        params: PluginActionParams,
        context: PluginContext,
    ) -> Option<Result<ActionOutput, PluginCallError>> {
        let plugin_id = self.action_to_plugin.get(params.name)?.clone();
        let plugin = self.plugins.get_mut(&plugin_id)?;
        Some(plugin.call_action(params, context))
    }

    /// Execute a command if a plugin provides it.
    /// Returns None if no plugin handles this command.
    pub fn execute_command(
        &mut self,
        command_name: &str,
        args: Vec<String>,
        context: PluginContext,
    ) -> Option<Result<ActionOutput, PluginCallError>> {
        let plugin_id = self.command_to_plugin.get(command_name)?.clone();
        let plugin = self.plugins.get_mut(&plugin_id)?;
        Some(plugin.call_command(command_name, args, context))
    }

    /// Fire a hook to all subscribed plugins.
    pub fn fire_hook(
        &mut self,
        hook_name: &str,
        extra: serde_json::Value,
        context: PluginContext,
    ) -> Vec<Result<(), PluginCallError>> {
        let subscriber_ids: Vec<String> = self.hook_subscribers
            .get(hook_name)
            .cloned()
            .unwrap_or_default();

        let mut results = Vec::new();
        for id in subscriber_ids {
            if let Some(plugin) = self.plugins.get_mut(&id) {
                // Clone the context for each plugin call.
                // We must clone because the context contains `HashMap` configuration which
                // needs to be passed to each plugin instance's `PluginHostContext`.
                let ctx_clone = PluginContext {
                    text: context.text,
                    selection: context.selection,
                    cursor: context.cursor,
                    config: context.config.clone(),
                };
                
                results.push(plugin.call_hook(hook_name, extra.clone(), ctx_clone));
            }
        }
        results
    }

}

impl<'a> Clone for PluginContext<'a> {
    fn clone(&self) -> Self {
        Self {
            text: self.text,
            selection: self.selection,
            cursor: self.cursor,
            config: self.config.clone(),
        }
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
