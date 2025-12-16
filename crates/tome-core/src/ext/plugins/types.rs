use serde::{Deserialize, Serialize};

/// JSON schema for editor state passed to plugins.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorState {
    pub text: String,
    pub cursor: usize,
    pub selection_anchor: usize,
    pub selection_head: usize,
}

/// JSON schema for plugin action input.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionInput {
    pub action_name: String,
    pub count: usize,
    pub extend: bool,
    pub char_arg: Option<char>,
    pub editor: EditorState,
}

/// JSON schema for plugin action output.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    pub open_file: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
}


/// JSON schema for hook input.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookInput {
    pub hook_name: String,
    pub editor: EditorState,
    #[serde(default)]
    pub extra: serde_json::Value,
}

/// JSON schema for command input.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandInput {
    pub command_name: String,
    pub args: Vec<String>,
    pub editor: EditorState,
}
