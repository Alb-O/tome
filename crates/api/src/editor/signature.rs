//! Signature help state machine for LSP signature help integration.
//!
//! This module handles the lifecycle of signature help requests, from triggering
//! on `(` to dismissing on `)` or cursor movement outside the call expression.

use std::time::Instant;

/// State of the LSP signature help system.
///
/// This state machine tracks the lifecycle of LSP signature help requests,
/// from triggering through display and dismissal.
#[derive(Debug, Clone, Default)]
pub enum SignatureHelpState {
	/// No signature help active.
	#[default]
	Inactive,

	/// Signature help request is pending.
	Requesting {
		/// Column position where signature help was triggered (at the `(`).
		trigger_column: usize,
		/// Current parameter index (based on comma count).
		parameter_index: usize,
		/// When the request was started.
		started_at: Instant,
	},

	/// Signature help popup is active and showing signatures.
	Active {
		/// Column position where signature help was triggered.
		trigger_column: usize,
		/// Current parameter index.
		parameter_index: usize,
		/// Nesting depth for nested function calls.
		nesting_depth: usize,
	},
}

impl SignatureHelpState {
	/// Returns whether signature help is currently active (showing popup).
	pub fn is_active(&self) -> bool {
		matches!(self, Self::Active { .. })
	}

	/// Returns whether a signature help request is pending.
	pub fn is_requesting(&self) -> bool {
		matches!(self, Self::Requesting { .. })
	}

	/// Returns whether signature help is inactive.
	pub fn is_inactive(&self) -> bool {
		matches!(self, Self::Inactive)
	}

	/// Returns the trigger column if signature help is active or requesting.
	pub fn trigger_column(&self) -> Option<usize> {
		match self {
			Self::Inactive => None,
			Self::Requesting { trigger_column, .. } => Some(*trigger_column),
			Self::Active { trigger_column, .. } => Some(*trigger_column),
		}
	}

	/// Returns the current parameter index.
	pub fn parameter_index(&self) -> Option<usize> {
		match self {
			Self::Inactive => None,
			Self::Requesting {
				parameter_index, ..
			} => Some(*parameter_index),
			Self::Active {
				parameter_index, ..
			} => Some(*parameter_index),
		}
	}

	/// Starts a signature help request.
	pub fn start_request(&mut self, trigger_column: usize) {
		*self = Self::Requesting {
			trigger_column,
			parameter_index: 0,
			started_at: Instant::now(),
		};
	}

	/// Transitions to active state after receiving signature help response.
	pub fn activate(&mut self) {
		if let Self::Requesting {
			trigger_column,
			parameter_index,
			..
		} = self
		{
			*self = Self::Active {
				trigger_column: *trigger_column,
				parameter_index: *parameter_index,
				nesting_depth: 1,
			};
		}
	}

	/// Increments the parameter index when a comma is typed.
	pub fn advance_parameter(&mut self) {
		match self {
			Self::Requesting {
				parameter_index, ..
			}
			| Self::Active {
				parameter_index, ..
			} => {
				*parameter_index += 1;
			}
			_ => {}
		}
	}

	/// Decrements the parameter index when backspace removes a comma.
	pub fn retreat_parameter(&mut self) {
		match self {
			Self::Requesting {
				parameter_index, ..
			}
			| Self::Active {
				parameter_index, ..
			} => {
				*parameter_index = parameter_index.saturating_sub(1);
			}
			_ => {}
		}
	}

	/// Increments nesting depth when `(` is typed inside a call.
	pub fn enter_nested(&mut self) {
		if let Self::Active { nesting_depth, .. } = self {
			*nesting_depth += 1;
		}
	}

	/// Decrements nesting depth when `)` is typed.
	/// Returns true if signature help should be dismissed (nesting reached 0).
	pub fn exit_nested(&mut self) -> bool {
		if let Self::Active { nesting_depth, .. } = self {
			*nesting_depth = nesting_depth.saturating_sub(1);
			if *nesting_depth == 0 {
				self.dismiss();
				return true;
			}
		}
		false
	}

	/// Resets to inactive state.
	pub fn dismiss(&mut self) {
		*self = Self::Inactive;
	}
}

/// Characters that trigger automatic signature help.
pub const SIGNATURE_TRIGGER_CHARS: &[char] = &['('];

/// Characters that re-trigger/update signature help.
pub const SIGNATURE_RETRIGGER_CHARS: &[char] = &[','];

/// Characters that dismiss signature help.
pub const SIGNATURE_CLOSE_CHARS: &[char] = &[')'];

/// Returns whether a character should trigger signature help.
pub fn is_signature_trigger_char(c: char) -> bool {
	SIGNATURE_TRIGGER_CHARS.contains(&c)
}

/// Returns whether a character should re-trigger/advance signature help.
pub fn is_signature_retrigger_char(c: char) -> bool {
	SIGNATURE_RETRIGGER_CHARS.contains(&c)
}

/// Returns whether a character should close signature help.
pub fn is_signature_close_char(c: char) -> bool {
	SIGNATURE_CLOSE_CHARS.contains(&c)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_signature_state_transitions() {
		let mut state = SignatureHelpState::Inactive;
		assert!(state.is_inactive());

		state.start_request(10);
		assert!(state.is_requesting());
		assert_eq!(state.trigger_column(), Some(10));
		assert_eq!(state.parameter_index(), Some(0));

		state.activate();
		assert!(state.is_active());
		assert_eq!(state.trigger_column(), Some(10));
		assert_eq!(state.parameter_index(), Some(0));

		state.advance_parameter();
		assert_eq!(state.parameter_index(), Some(1));

		state.advance_parameter();
		assert_eq!(state.parameter_index(), Some(2));

		state.retreat_parameter();
		assert_eq!(state.parameter_index(), Some(1));

		state.dismiss();
		assert!(state.is_inactive());
	}

	#[test]
	fn test_nesting() {
		let mut state = SignatureHelpState::Active {
			trigger_column: 5,
			parameter_index: 0,
			nesting_depth: 1,
		};

		// Enter nested call
		state.enter_nested();
		if let SignatureHelpState::Active { nesting_depth, .. } = &state {
			assert_eq!(*nesting_depth, 2);
		}

		// Exit nested call (shouldn't dismiss yet)
		let dismissed = state.exit_nested();
		assert!(!dismissed);
		assert!(state.is_active());

		// Exit outer call (should dismiss)
		let dismissed = state.exit_nested();
		assert!(dismissed);
		assert!(state.is_inactive());
	}

	#[test]
	fn test_trigger_chars() {
		assert!(is_signature_trigger_char('('));
		assert!(!is_signature_trigger_char('['));
		assert!(!is_signature_trigger_char('a'));

		assert!(is_signature_retrigger_char(','));
		assert!(!is_signature_retrigger_char(';'));

		assert!(is_signature_close_char(')'));
		assert!(!is_signature_close_char(']'));
	}
}
