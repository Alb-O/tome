//! Completion state machine for LSP completion integration.
//!
//! This module handles the lifecycle of completion requests, from triggering
//! to accepting or dismissing completion items.

use std::time::Instant;

/// State of the LSP completion system.
///
/// This state machine tracks the lifecycle of LSP completion requests,
/// from triggering through display and acceptance.
#[derive(Debug, Clone, Default)]
pub enum LspCompletionState {
	/// No completion active.
	#[default]
	Inactive,

	/// Completion request is pending.
	Requesting {
		/// Column position where completion was triggered.
		trigger_column: usize,
		/// Text typed since trigger.
		typed_text: String,
		/// When the request was started.
		started_at: Instant,
	},

	/// Completion popup is active and showing items.
	Active {
		/// Column position where completion was triggered.
		trigger_column: usize,
		/// Text typed since trigger (for filtering).
		typed_text: String,
	},

	/// A completion item is being inserted.
	Inserting {
		/// Column position where completion was triggered.
		trigger_column: usize,
	},
}

impl LspCompletionState {
	/// Returns whether completion is currently active (showing popup).
	pub fn is_active(&self) -> bool {
		matches!(self, Self::Active { .. })
	}

	/// Returns whether a completion request is pending.
	pub fn is_requesting(&self) -> bool {
		matches!(self, Self::Requesting { .. })
	}

	/// Returns whether completion is inactive.
	pub fn is_inactive(&self) -> bool {
		matches!(self, Self::Inactive)
	}

	/// Returns the trigger column if completion is active or requesting.
	pub fn trigger_column(&self) -> Option<usize> {
		match self {
			Self::Inactive => None,
			Self::Requesting { trigger_column, .. } => Some(*trigger_column),
			Self::Active { trigger_column, .. } => Some(*trigger_column),
			Self::Inserting { trigger_column } => Some(*trigger_column),
		}
	}

	/// Returns the typed text if completion is active or requesting.
	pub fn typed_text(&self) -> Option<&str> {
		match self {
			Self::Inactive => None,
			Self::Requesting { typed_text, .. } => Some(typed_text),
			Self::Active { typed_text, .. } => Some(typed_text),
			Self::Inserting { .. } => None,
		}
	}

	/// Starts a completion request.
	pub fn start_request(&mut self, trigger_column: usize, typed_text: String) {
		*self = Self::Requesting {
			trigger_column,
			typed_text,
			started_at: Instant::now(),
		};
	}

	/// Transitions to active state after receiving completion response.
	pub fn activate(&mut self) {
		if let Self::Requesting {
			trigger_column,
			typed_text,
			..
		} = self
		{
			*self = Self::Active {
				trigger_column: *trigger_column,
				typed_text: typed_text.clone(),
			};
		}
	}

	/// Updates the typed text for filtering when user types.
	pub fn update_typed_text(&mut self, text: String) {
		match self {
			Self::Requesting { typed_text, .. } | Self::Active { typed_text, .. } => {
				*typed_text = text;
			}
			_ => {}
		}
	}

	/// Transitions to inserting state when accepting a completion.
	pub fn start_insert(&mut self) {
		if let Self::Active { trigger_column, .. } = self {
			*self = Self::Inserting {
				trigger_column: *trigger_column,
			};
		}
	}

	/// Resets to inactive state.
	pub fn dismiss(&mut self) {
		*self = Self::Inactive;
	}
}

/// Characters that trigger automatic completion.
pub const TRIGGER_CHARS: &[char] = &['.', ':', '(', '<'];

/// Returns whether a character should trigger completion.
pub fn is_trigger_char(c: char) -> bool {
	TRIGGER_CHARS.contains(&c)
}

/// Returns whether completion should be re-triggered on backspace.
///
/// This returns true if the character deleted was part of the current
/// filter text but we're still past the trigger column.
pub fn should_retrigger_on_backspace(state: &LspCompletionState, cursor_column: usize) -> bool {
	match state {
		LspCompletionState::Active { trigger_column, .. } => cursor_column > *trigger_column,
		_ => false,
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_completion_state_transitions() {
		let mut state = LspCompletionState::Inactive;
		assert!(state.is_inactive());

		state.start_request(5, "fo".to_string());
		assert!(state.is_requesting());
		assert_eq!(state.trigger_column(), Some(5));
		assert_eq!(state.typed_text(), Some("fo"));

		state.activate();
		assert!(state.is_active());
		assert_eq!(state.trigger_column(), Some(5));

		state.update_typed_text("foo".to_string());
		assert_eq!(state.typed_text(), Some("foo"));

		state.start_insert();
		assert!(!state.is_active());
		assert_eq!(state.trigger_column(), Some(5));

		state.dismiss();
		assert!(state.is_inactive());
	}

	#[test]
	fn test_trigger_chars() {
		assert!(is_trigger_char('.'));
		assert!(is_trigger_char(':'));
		assert!(is_trigger_char('('));
		assert!(!is_trigger_char('a'));
		assert!(!is_trigger_char(' '));
	}

	#[test]
	fn test_retrigger_on_backspace() {
		let state = LspCompletionState::Active {
			trigger_column: 5,
			typed_text: "foo".to_string(),
		};

		assert!(should_retrigger_on_backspace(&state, 7)); // Still past trigger
		assert!(should_retrigger_on_backspace(&state, 6)); // Still past trigger
		assert!(!should_retrigger_on_backspace(&state, 5)); // At trigger
		assert!(!should_retrigger_on_backspace(&state, 4)); // Before trigger
	}
}
