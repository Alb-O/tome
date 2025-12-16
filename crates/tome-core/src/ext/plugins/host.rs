//! Host functions exposed to plugins.
//!
//! These functions allow plugins to interact with the editor via the Extism runtime.

use extism::UserData;

use crate::selection::Selection;

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
}

/// Operations queued by plugins to be applied after the call completes.
#[derive(Debug, Clone)]
pub enum PendingOp {
    SetCursor(usize),
    SetSelection { anchor: usize, head: usize },
    Insert(String),
    Delete,
    Message(String),
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
        }
    }

    pub fn update(&mut self, text: String, selection: &Selection, cursor: usize) {
        let primary = selection.primary();
        self.text = text;
        self.selection_anchor = primary.anchor;
        self.selection_head = primary.head;
        self.cursor = cursor;
        self.pending_ops.clear();
        self.messages.clear();
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

/// JSON schema for editor state passed to plugins.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EditorState {
    pub text: String,
    pub cursor: usize,
    pub selection_anchor: usize,
    pub selection_head: usize,
}

impl EditorState {
    pub fn from_context(ctx: &PluginHostContext) -> Self {
        Self {
            text: ctx.text.clone(),
            cursor: ctx.cursor,
            selection_anchor: ctx.selection_anchor,
            selection_head: ctx.selection_head,
        }
    }
}

/// JSON schema for plugin action input.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionInput {
    pub action_name: String,
    pub count: usize,
    pub extend: bool,
    pub char_arg: Option<char>,
    pub editor: EditorState,
}

/// JSON schema for plugin action output.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ActionOutput {
    #[serde(default)]
    pub set_cursor: Option<usize>,
    #[serde(default)]
    pub set_selection: Option<(usize, usize)>,
    #[serde(default)]
    pub insert_text: Option<String>,
    #[serde(default)]
    pub delete: bool,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
}

/// JSON schema for hook input.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HookInput {
    pub hook_name: String,
    pub editor: EditorState,
    #[serde(default)]
    pub extra: serde_json::Value,
}

/// JSON schema for command input.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommandInput {
    pub command_name: String,
    pub args: Vec<String>,
    pub editor: EditorState,
}

// Host functions are defined using extism's host_fn! macro.
// The user_data type in the macro is what UserData<T>::get() returns after locking,
// which is the inner type T.

/// Get the document text.
/// Returns: the text as a string
extism::host_fn!(pub editor_get_text(user_data: PluginHostContext;) -> String {
    let ctx = user_data.get().map_err(|e| extism::Error::msg(e.to_string()))?;
    let ctx = ctx.lock().map_err(|e| extism::Error::msg(e.to_string()))?;
    Ok(ctx.text.clone())
});

/// Get the current cursor position.
/// Returns: cursor position as u64
extism::host_fn!(pub editor_get_cursor(user_data: PluginHostContext;) -> u64 {
    let ctx = user_data.get().map_err(|e| extism::Error::msg(e.to_string()))?;
    let ctx = ctx.lock().map_err(|e| extism::Error::msg(e.to_string()))?;
    Ok(ctx.cursor as u64)
});

/// Set the cursor position.
/// Input: cursor position as u64
extism::host_fn!(pub editor_set_cursor(user_data: PluginHostContext; pos: u64) {
    let ctx = user_data.get().map_err(|e| extism::Error::msg(e.to_string()))?;
    let mut ctx = ctx.lock().map_err(|e| extism::Error::msg(e.to_string()))?;
    ctx.pending_ops.push(PendingOp::SetCursor(pos as usize));
    Ok(())
});

/// Get the current selection as JSON: {"anchor": n, "head": n}
extism::host_fn!(pub editor_get_selection(user_data: PluginHostContext;) -> String {
    let ctx = user_data.get().map_err(|e| extism::Error::msg(e.to_string()))?;
    let ctx = ctx.lock().map_err(|e| extism::Error::msg(e.to_string()))?;
    let json = serde_json::json!({
        "anchor": ctx.selection_anchor,
        "head": ctx.selection_head
    });
    Ok(json.to_string())
});

/// Set the selection from JSON: {"anchor": n, "head": n}
extism::host_fn!(pub editor_set_selection(user_data: PluginHostContext; json: String) {
    let ctx = user_data.get().map_err(|e| extism::Error::msg(e.to_string()))?;
    let mut ctx = ctx.lock().map_err(|e| extism::Error::msg(e.to_string()))?;
    
    #[derive(serde::Deserialize)]
    struct SelectionJson {
        anchor: usize,
        head: usize,
    }
    
    let sel: SelectionJson = serde_json::from_str(&json)
        .map_err(|e| extism::Error::msg(format!("invalid selection JSON: {}", e)))?;
    
    ctx.pending_ops.push(PendingOp::SetSelection {
        anchor: sel.anchor,
        head: sel.head,
    });
    Ok(())
});

/// Insert text at the current cursor position.
extism::host_fn!(pub editor_insert(user_data: PluginHostContext; text: String) {
    let ctx = user_data.get().map_err(|e| extism::Error::msg(e.to_string()))?;
    let mut ctx = ctx.lock().map_err(|e| extism::Error::msg(e.to_string()))?;
    ctx.pending_ops.push(PendingOp::Insert(text));
    Ok(())
});

/// Delete the current selection.
extism::host_fn!(pub editor_delete(user_data: PluginHostContext;) {
    let ctx = user_data.get().map_err(|e| extism::Error::msg(e.to_string()))?;
    let mut ctx = ctx.lock().map_err(|e| extism::Error::msg(e.to_string()))?;
    ctx.pending_ops.push(PendingOp::Delete);
    Ok(())
});

/// Show a message to the user.
extism::host_fn!(pub editor_message(user_data: PluginHostContext; msg: String) {
    let ctx = user_data.get().map_err(|e| extism::Error::msg(e.to_string()))?;
    let mut ctx = ctx.lock().map_err(|e| extism::Error::msg(e.to_string()))?;
    ctx.messages.push(msg);
    Ok(())
});

/// Build the list of host functions for plugin creation.
pub fn create_host_functions(ctx: SharedHostContext) -> Vec<extism::Function> {
    use extism::ValType;
    
    // PTR is ValType::I64 - represents a pointer to memory
    const PTR: extism::ValType = extism::ValType::I64;
    
    vec![
        extism::Function::new(
            "editor_get_text",
            [],
            [PTR],
            ctx.clone(),
            editor_get_text,
        ),
        extism::Function::new(
            "editor_get_cursor",
            [],
            [ValType::I64],
            ctx.clone(),
            editor_get_cursor,
        ),
        extism::Function::new(
            "editor_set_cursor",
            [ValType::I64],
            [],
            ctx.clone(),
            editor_set_cursor,
        ),
        extism::Function::new(
            "editor_get_selection",
            [],
            [PTR],
            ctx.clone(),
            editor_get_selection,
        ),
        extism::Function::new(
            "editor_set_selection",
            [PTR],
            [],
            ctx.clone(),
            editor_set_selection,
        ),
        extism::Function::new(
            "editor_insert",
            [PTR],
            [],
            ctx.clone(),
            editor_insert,
        ),
        extism::Function::new(
            "editor_delete",
            [],
            [],
            ctx.clone(),
            editor_delete,
        ),
        extism::Function::new(
            "editor_message",
            [PTR],
            [],
            ctx,
            editor_message,
        ),
    ]
}
