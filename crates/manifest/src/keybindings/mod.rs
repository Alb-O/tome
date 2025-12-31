//! Keybinding registration system.
//!
//! Keybindings map key sequences to actions in different modes. Uses a trie-based
//! registry for efficient sequence matching (e.g., `g g` for document_start).
//!
//! All keybindings are colocated with their action definitions using the `action!`
//! macro with `bindings:` syntax:
//!
//! ```ignore
//! action!(
//!     document_start,
//!     {
//!         description: "Move to document start",
//!         bindings: r#"
//!             normal "g g" "ctrl-home"
//!             insert "ctrl-home"
//!         "#
//!     },
//!     |_ctx| { ... }
//! );
//! ```

use linkme::distributed_slice;

use crate::Mode;

/// Distributed slice for key sequence bindings.
///
/// Populated at compile time by the `action!` macro's `bindings:` syntax.
#[distributed_slice]
pub static KEYBINDINGS: [KeyBindingDef];

/// Key sequence binding definition.
///
/// Maps a key sequence (e.g., `"g g"`, `"ctrl-w s"`) to an action in a mode.
#[derive(Clone, Copy)]
pub struct KeyBindingDef {
	/// Mode this binding is active in.
	pub mode: BindingMode,
	/// Key sequence string (e.g., `"g g"`, `"ctrl-home"`).
	/// Parsed with `parse_seq()` at registry initialization.
	pub keys: &'static str,
	/// Action to execute (looked up by name in the action registry).
	pub action: &'static str,
	/// Priority for conflict resolution (lower wins).
	/// Default bindings use 100; user overrides should use lower values.
	pub priority: i16,
}

impl std::fmt::Debug for KeyBindingDef {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("KeyBindingDef")
			.field("mode", &self.mode)
			.field("keys", &self.keys)
			.field("action", &self.action)
			.field("priority", &self.priority)
			.finish()
	}
}

/// Mode in which a keybinding is active.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BindingMode {
	/// Normal mode (default editing mode).
	Normal,
	/// Insert mode (text input).
	Insert,
	/// Match mode (m prefix).
	Match,
	/// Window mode (Ctrl+w prefix).
	Window,
	/// Space mode (space prefix).
	Space,
}

impl From<Mode> for BindingMode {
	fn from(mode: Mode) -> Self {
		match mode {
			Mode::Normal => BindingMode::Normal,
			Mode::Insert => BindingMode::Insert,
			Mode::Window => BindingMode::Window,
			Mode::PendingAction(_) => BindingMode::Normal,
		}
	}
}
