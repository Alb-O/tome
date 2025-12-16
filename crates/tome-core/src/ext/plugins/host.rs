//! Host functions exposed to plugins.
//!
//! These functions allow plugins to interact with the editor.

#![allow(dead_code)] // Stub types for future extism integration

use std::sync::{Arc, Mutex};

use crate::selection::Selection;

/// Context passed to host functions, providing access to editor state.
pub struct PluginHostContext {
    /// Document text (copy for safety).
    pub text: String,
    /// Current selection.
    pub selection: Selection,
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
    SetSelection(Selection),
    Insert(String),
    Delete,
    Message(String),
}

impl PluginHostContext {
    pub fn new(text: String, selection: Selection, cursor: usize) -> Self {
        Self {
            text,
            selection,
            cursor,
            pending_ops: Vec::new(),
            messages: Vec::new(),
        }
    }
}

/// Shared context for thread-safe access from host functions.
pub type SharedHostContext = Arc<Mutex<PluginHostContext>>;

// Host function implementations will be added when extism is integrated.
// For now, we define the interface.

/// JSON schema for editor state passed to plugins.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EditorState {
    pub text: String,
    pub cursor: usize,
    pub selection_anchor: usize,
    pub selection_head: usize,
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
