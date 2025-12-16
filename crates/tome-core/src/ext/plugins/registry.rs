//! Plugin registry - manages loaded plugins and their registrations.

#![allow(dead_code)] // Stub types for future extism integration

use std::collections::HashMap;
use std::path::PathBuf;

/// A loaded plugin with its metadata and registrations.
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
    // Note: The actual extism::Plugin will be stored here when integrated.
    // For now we just store metadata.
}

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

    /// List all loaded plugins.
    pub fn list(&self) -> impl Iterator<Item = &LoadedPlugin> {
        self.plugins.values()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}
