//! Action system for extensible commands and motions.
//!
//! Actions are the unified abstraction for all editor operations that can be
//! triggered by keybindings. This replaces the hardcoded `Command` enum with
//! a dynamic, extensible registry.

mod modes;
mod motions;

use linkme::distributed_slice;
use ropey::RopeSlice;

use crate::selection::Selection;

/// Registry of all actions, populated at link time.
#[distributed_slice]
pub static ACTIONS: [ActionDef];

/// The result of executing an action.
#[derive(Debug, Clone)]
pub enum ActionResult {
    /// Action completed successfully, no mode change.
    Ok,
    /// Action requests a mode change.
    ModeChange(ActionMode),
    /// Action is a motion that produces a new selection.
    Motion(Selection),
    /// Action requests quitting the editor.
    Quit,
    /// Action requests quitting without saving.
    ForceQuit,
    /// Action failed with an error message.
    Error(String),
    /// Action needs more input (e.g., awaiting a character for 'f' find).
    Pending(PendingAction),
}

/// Mode to switch to after an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionMode {
    Normal,
    Insert,
    Goto,
    View,
    Command,
}

/// An action that needs additional input to complete.
#[derive(Debug, Clone)]
pub struct PendingAction {
    pub action: &'static str,
    pub prompt: String,
}

/// Context passed to action handlers.
pub struct ActionContext<'a> {
    /// The document text.
    pub text: RopeSlice<'a>,
    /// Current selection.
    pub selection: &'a Selection,
    /// Count prefix (defaults to 1).
    pub count: usize,
    /// Whether to extend selection instead of moving.
    pub extend: bool,
    /// Register name (if any).
    pub register: Option<char>,
    /// Additional arguments (e.g., character for find).
    pub args: ActionArgs,
}

/// Additional arguments for actions.
#[derive(Debug, Clone, Default)]
pub struct ActionArgs {
    /// Character argument (for f/t/r commands).
    pub char: Option<char>,
    /// String argument (for search, etc.).
    pub string: Option<String>,
}

/// Definition of an action that can be registered.
pub struct ActionDef {
    /// Unique name for the action (e.g., "move_left", "delete_selection").
    pub name: &'static str,
    /// Human-readable description.
    pub description: &'static str,
    /// The action handler function.
    pub handler: ActionHandler,
}

/// The type of action handler functions.
pub type ActionHandler = fn(&ActionContext) -> ActionResult;

/// Look up an action by name.
pub fn find_action(name: &str) -> Option<&'static ActionDef> {
    ACTIONS.iter().find(|a| a.name == name)
}

/// Execute an action by name with the given context.
pub fn execute_action(name: &str, ctx: &ActionContext) -> ActionResult {
    match find_action(name) {
        Some(action) => (action.handler)(ctx),
        None => ActionResult::Error(format!("Unknown action: {}", name)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_action_unknown() {
        assert!(find_action("nonexistent_action_xyz").is_none());
    }

    #[test]
    fn test_motion_actions_registered() {
        assert!(find_action("move_left").is_some());
        assert!(find_action("move_right").is_some());
        assert!(find_action("move_up").is_some());
        assert!(find_action("move_down").is_some());
        assert!(find_action("move_line_start").is_some());
        assert!(find_action("move_line_end").is_some());
        assert!(find_action("next_word_start").is_some());
        assert!(find_action("document_start").is_some());
        assert!(find_action("document_end").is_some());
    }
}
