//! Core notification catalog with typed keys for compile-time checked notifications.

use std::path::Path;
use std::time::Duration;

use linkme::distributed_slice;

use crate::{AutoDismiss, Level, Notification, NotificationDef, NotificationKey, NOTIFICATIONS};

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_BUFFER_READONLY: NotificationDef =
	NotificationDef::new("buffer_readonly", Level::Warn, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_NOTHING_TO_UNDO: NotificationDef =
	NotificationDef::new("nothing_to_undo", Level::Warn, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_NOTHING_TO_REDO: NotificationDef =
	NotificationDef::new("nothing_to_redo", Level::Warn, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_UNDO: NotificationDef =
	NotificationDef::new("undo", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_REDO: NotificationDef =
	NotificationDef::new("redo", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_SEARCH_WRAPPED: NotificationDef =
	NotificationDef::new("search_wrapped", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_NO_SEARCH_PATTERN: NotificationDef =
	NotificationDef::new("no_search_pattern", Level::Warn, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_NO_SELECTION: NotificationDef =
	NotificationDef::new("no_selection", Level::Warn, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_NO_MORE_MATCHES: NotificationDef =
	NotificationDef::new("no_more_matches", Level::Warn, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_NO_MATCHES_FOUND: NotificationDef =
	NotificationDef::new("no_matches_found", Level::Warn, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_NO_BUFFERS: NotificationDef =
	NotificationDef::new("no_buffers", Level::Warn, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_BUFFER_MODIFIED: NotificationDef =
	NotificationDef::new("buffer_modified", Level::Warn, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_NO_SELECTIONS_REMAIN: NotificationDef =
	NotificationDef::new("no_selections_remain", Level::Warn, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_YANKED_CHARS: NotificationDef =
	NotificationDef::new("yanked_chars", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_YANKED_LINES: NotificationDef =
	NotificationDef::new("yanked_lines", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_DELETED_CHARS: NotificationDef =
	NotificationDef::new("deleted_chars", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_PATTERN_NOT_FOUND: NotificationDef =
	NotificationDef::new("pattern_not_found", Level::Warn, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_REGEX_ERROR: NotificationDef = NotificationDef::new(
	"regex_error",
	Level::Error,
	AutoDismiss::After(Duration::from_secs(8)),
	crate::RegistrySource::Builtin,
);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_SEARCH_INFO: NotificationDef =
	NotificationDef::new("search_info", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_REPLACED: NotificationDef =
	NotificationDef::new("replaced", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_MATCHES_COUNT: NotificationDef =
	NotificationDef::new("matches_count", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_SPLITS_COUNT: NotificationDef =
	NotificationDef::new("splits_count", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_SELECTIONS_KEPT: NotificationDef =
	NotificationDef::new("selections_kept", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_FILE_SAVED: NotificationDef =
	NotificationDef::new("file_saved", Level::Success, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_FILE_NOT_FOUND: NotificationDef =
	NotificationDef::new("file_not_found", Level::Error, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_FILE_LOAD_ERROR: NotificationDef =
	NotificationDef::new("file_load_error", Level::Error, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_FILE_SAVE_ERROR: NotificationDef =
	NotificationDef::new("file_save_error", Level::Error, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_BUFFER_CLOSED: NotificationDef =
	NotificationDef::new("buffer_closed", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_UNKNOWN_COMMAND: NotificationDef =
	NotificationDef::new("unknown_command", Level::Error, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_COMMAND_ERROR: NotificationDef =
	NotificationDef::new("command_error", Level::Error, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_UNKNOWN_ACTION: NotificationDef =
	NotificationDef::new("unknown_action", Level::Error, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_ACTION_ERROR: NotificationDef =
	NotificationDef::new("action_error", Level::Error, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_THEME_SET: NotificationDef =
	NotificationDef::new("theme_set", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_SPLIT_NO_RANGES: NotificationDef =
	NotificationDef::new("split_no_ranges", Level::Warn, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_NO_MATCHES_TO_SPLIT: NotificationDef =
	NotificationDef::new("no_matches_to_split", Level::Warn, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_READONLY_ENABLED: NotificationDef =
	NotificationDef::new("readonly_enabled", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_READONLY_DISABLED: NotificationDef =
	NotificationDef::new("readonly_disabled", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_UNSAVED_CHANGES_FORCE_QUIT: NotificationDef =
	NotificationDef::new("unsaved_changes_force_quit", Level::Error, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_NOT_IMPLEMENTED: NotificationDef =
	NotificationDef::new("not_implemented", Level::Warn, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_VIEWPORT_UNAVAILABLE: NotificationDef =
	NotificationDef::new("viewport_unavailable", Level::Error, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_SCREEN_MOTION_UNAVAILABLE: NotificationDef =
	NotificationDef::new("screen_motion_unavailable", Level::Error, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_HELP_TEXT: NotificationDef =
	NotificationDef::new("help_text", Level::Info, AutoDismiss::Never, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_DIAGNOSTIC_OUTPUT: NotificationDef =
	NotificationDef::new("diagnostic_output", Level::Info, AutoDismiss::Never, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_DIAGNOSTIC_WARNING: NotificationDef =
	NotificationDef::new("diagnostic_warning", Level::Warn, AutoDismiss::Never, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_NO_COLLISIONS: NotificationDef =
	NotificationDef::new("no_collisions", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_PENDING_PROMPT: NotificationDef =
	NotificationDef::new("pending_prompt", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_COUNT_DISPLAY: NotificationDef =
	NotificationDef::new("count_display", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_ACP_STARTING: NotificationDef =
	NotificationDef::new("acp_starting", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_ACP_STOPPED: NotificationDef =
	NotificationDef::new("acp_stopped", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_ACP_CANCELLED: NotificationDef =
	NotificationDef::new("acp_cancelled", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_ACP_MODEL_SET: NotificationDef =
	NotificationDef::new("acp_model_set", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_ACP_MODEL_INFO: NotificationDef =
	NotificationDef::new("acp_model_info", Level::Info, AutoDismiss::Never, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_INFO: NotificationDef =
	NotificationDef::new("info", Level::Info, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_WARN: NotificationDef =
	NotificationDef::new("warn", Level::Warn, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_ERROR: NotificationDef =
	NotificationDef::new("error", Level::Error, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_SUCCESS: NotificationDef =
	NotificationDef::new("success", Level::Success, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_DEBUG: NotificationDef =
	NotificationDef::new("debug", Level::Debug, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

#[distributed_slice(NOTIFICATIONS)]
static NOTIF_UNHANDLED_RESULT: NotificationDef =
	NotificationDef::new("unhandled_result", Level::Debug, AutoDismiss::DEFAULT, crate::RegistrySource::Builtin);

/// Typed notification keys for compile-time checked notifications.
#[allow(non_upper_case_globals, non_camel_case_types)]
pub mod keys {
	use super::*;

	pub const buffer_readonly: NotificationKey = NotificationKey::new(&NOTIF_BUFFER_READONLY, "Buffer is read-only");
	pub const nothing_to_undo: NotificationKey = NotificationKey::new(&NOTIF_NOTHING_TO_UNDO, "Nothing to undo");
	pub const nothing_to_redo: NotificationKey = NotificationKey::new(&NOTIF_NOTHING_TO_REDO, "Nothing to redo");
	pub const undo: NotificationKey = NotificationKey::new(&NOTIF_UNDO, "Undo");
	pub const redo: NotificationKey = NotificationKey::new(&NOTIF_REDO, "Redo");
	pub const search_wrapped: NotificationKey = NotificationKey::new(&NOTIF_SEARCH_WRAPPED, "Search wrapped to beginning");
	pub const no_search_pattern: NotificationKey = NotificationKey::new(&NOTIF_NO_SEARCH_PATTERN, "No search pattern");
	pub const no_selection: NotificationKey = NotificationKey::new(&NOTIF_NO_SELECTION, "No selection");
	pub const no_more_matches: NotificationKey = NotificationKey::new(&NOTIF_NO_MORE_MATCHES, "No more matches");
	pub const no_matches_found: NotificationKey = NotificationKey::new(&NOTIF_NO_MATCHES_FOUND, "No matches found");
	pub const no_buffers: NotificationKey = NotificationKey::new(&NOTIF_NO_BUFFERS, "No buffers open");
	pub const buffer_modified: NotificationKey = NotificationKey::new(&NOTIF_BUFFER_MODIFIED, "Buffer has unsaved changes");
	pub const no_selections_remain: NotificationKey = NotificationKey::new(&NOTIF_NO_SELECTIONS_REMAIN, "No selections remain");
	pub const pattern_not_found: NotificationKey = NotificationKey::new(&NOTIF_PATTERN_NOT_FOUND, "Pattern not found");
	pub const no_selection_to_search: NotificationKey = NotificationKey::new(&NOTIF_NO_SELECTION, "No selection to search in");
	pub const no_selection_to_split: NotificationKey = NotificationKey::new(&NOTIF_NO_SELECTION, "No selection to split");
	pub const split_no_ranges: NotificationKey = NotificationKey::new(&NOTIF_SPLIT_NO_RANGES, "Split produced no ranges");
	pub const no_matches_to_split: NotificationKey = NotificationKey::new(&NOTIF_NO_MATCHES_TO_SPLIT, "No matches found to split on");
	pub const readonly_enabled: NotificationKey = NotificationKey::new(&NOTIF_READONLY_ENABLED, "Read-only enabled");
	pub const readonly_disabled: NotificationKey = NotificationKey::new(&NOTIF_READONLY_DISABLED, "Read-only disabled");
	pub const unsaved_changes_force_quit: NotificationKey =
		NotificationKey::new(&NOTIF_UNSAVED_CHANGES_FORCE_QUIT, "Buffer has unsaved changes (use :q! to force quit)");
	pub const viewport_unavailable: NotificationKey =
		NotificationKey::new(&NOTIF_VIEWPORT_UNAVAILABLE, "Viewport info unavailable for screen motion");
	pub const viewport_height_unavailable: NotificationKey =
		NotificationKey::new(&NOTIF_VIEWPORT_UNAVAILABLE, "Viewport height unavailable for screen motion");
	pub const screen_motion_unavailable: NotificationKey =
		NotificationKey::new(&NOTIF_SCREEN_MOTION_UNAVAILABLE, "Screen motion target is unavailable");
	pub const no_collisions: NotificationKey = NotificationKey::new(&NOTIF_NO_COLLISIONS, "All good! No collisions found.");
	pub const acp_starting: NotificationKey = NotificationKey::new(&NOTIF_ACP_STARTING, "ACP agent starting...");
	pub const acp_stopped: NotificationKey = NotificationKey::new(&NOTIF_ACP_STOPPED, "ACP agent stopped");
	pub const acp_cancelled: NotificationKey = NotificationKey::new(&NOTIF_ACP_CANCELLED, "ACP request cancelled");

	/// Parameterized notification: "Yanked N chars".
	pub struct yanked_chars;
	impl yanked_chars {
		pub fn call(count: usize) -> Notification {
			Notification::new(&NOTIF_YANKED_CHARS, format!("Yanked {} chars", count))
		}
	}

	/// Parameterized notification: "Yanked N lines".
	pub struct yanked_lines;
	impl yanked_lines {
		pub fn call(count: usize) -> Notification {
			Notification::new(&NOTIF_YANKED_LINES, format!("Yanked {} lines", count))
		}
	}

	/// Parameterized notification: "Deleted N chars".
	pub struct deleted_chars;
	impl deleted_chars {
		pub fn call(count: usize) -> Notification {
			Notification::new(&NOTIF_DELETED_CHARS, format!("Deleted {} chars", count))
		}
	}

	/// Parameterized notification: "Pattern 'X' not found".
	pub struct pattern_not_found_with;
	impl pattern_not_found_with {
		pub fn call(pattern: &str) -> Notification {
			Notification::new(&NOTIF_PATTERN_NOT_FOUND, format!("Pattern '{}' not found", pattern))
		}
	}

	/// Parameterized notification for regex compilation errors.
	pub struct regex_error;
	impl regex_error {
		pub fn call(err: &str) -> Notification {
			Notification::new(&NOTIF_REGEX_ERROR, format!("Regex error: {}", err))
		}
	}

	/// Parameterized notification: "Search: X".
	pub struct search_info;
	impl search_info {
		pub fn call(text: &str) -> Notification {
			Notification::new(&NOTIF_SEARCH_INFO, format!("Search: {}", text))
		}
	}

	/// Parameterized notification: "Replaced N occurrences".
	pub struct replaced;
	impl replaced {
		pub fn call(count: usize) -> Notification {
			Notification::new(&NOTIF_REPLACED, format!("Replaced {} occurrences", count))
		}
	}

	/// Parameterized notification: "N matches".
	pub struct matches_count;
	impl matches_count {
		pub fn call(count: usize) -> Notification {
			Notification::new(&NOTIF_MATCHES_COUNT, format!("{} matches", count))
		}
	}

	/// Parameterized notification: "N splits".
	pub struct splits_count;
	impl splits_count {
		pub fn call(count: usize) -> Notification {
			Notification::new(&NOTIF_SPLITS_COUNT, format!("{} splits", count))
		}
	}

	/// Parameterized notification: "N selections kept".
	pub struct selections_kept;
	impl selections_kept {
		pub fn call(count: usize) -> Notification {
			Notification::new(&NOTIF_SELECTIONS_KEPT, format!("{} selections kept", count))
		}
	}

	/// Parameterized notification: "Saved /path/to/file".
	pub struct file_saved;
	impl file_saved {
		pub fn call(path: &Path) -> Notification {
			Notification::new(&NOTIF_FILE_SAVED, format!("Saved {}", path.display()))
		}
	}

	/// Parameterized notification: "File not found: /path".
	pub struct file_not_found;
	impl file_not_found {
		pub fn call(path: &Path) -> Notification {
			Notification::new(&NOTIF_FILE_NOT_FOUND, format!("File not found: {}", path.display()))
		}
	}

	/// Parameterized notification for file load errors.
	pub struct file_load_error;
	impl file_load_error {
		pub fn call(err: &str) -> Notification {
			Notification::new(&NOTIF_FILE_LOAD_ERROR, format!("Failed to load file: {}", err))
		}
	}

	/// Parameterized notification for file save errors.
	pub struct file_save_error;
	impl file_save_error {
		pub fn call(err: &str) -> Notification {
			Notification::new(&NOTIF_FILE_SAVE_ERROR, format!("Failed to save: {}", err))
		}
	}

	/// Parameterized notification: "Closed name".
	pub struct buffer_closed;
	impl buffer_closed {
		pub fn call(name: &str) -> Notification {
			Notification::new(&NOTIF_BUFFER_CLOSED, format!("Closed {}", name))
		}
	}

	/// Parameterized notification: "Unknown command: X".
	pub struct unknown_command;
	impl unknown_command {
		pub fn call(cmd: &str) -> Notification {
			Notification::new(&NOTIF_UNKNOWN_COMMAND, format!("Unknown command: {}", cmd))
		}
	}

	/// Parameterized notification for command execution errors.
	pub struct command_error;
	impl command_error {
		pub fn call(err: &str) -> Notification {
			Notification::new(&NOTIF_COMMAND_ERROR, format!("Command failed: {}", err))
		}
	}

	/// Parameterized notification: "Unknown action: X".
	pub struct unknown_action;
	impl unknown_action {
		pub fn call(name: &str) -> Notification {
			Notification::new(&NOTIF_UNKNOWN_ACTION, format!("Unknown action: {}", name))
		}
	}

	/// Parameterized notification for action execution errors.
	pub struct action_error;
	impl action_error {
		pub fn call(err: impl core::fmt::Display) -> Notification {
			Notification::new(&NOTIF_ACTION_ERROR, err.to_string())
		}
	}

	/// Parameterized notification: "Theme set to 'X'".
	pub struct theme_set;
	impl theme_set {
		pub fn call(name: &str) -> Notification {
			Notification::new(&NOTIF_THEME_SET, format!("Theme set to '{}'", name))
		}
	}

	/// Parameterized notification: "X - not yet implemented".
	pub struct not_implemented;
	impl not_implemented {
		pub fn call(feature: &str) -> Notification {
			Notification::new(&NOTIF_NOT_IMPLEMENTED, format!("{} - not yet implemented", feature))
		}
	}

	/// Multi-line help text (no auto-dismiss).
	pub struct help_text;
	impl help_text {
		pub fn call(text: impl Into<String>) -> Notification {
			Notification::new(&NOTIF_HELP_TEXT, text)
		}
	}

	/// Multi-line diagnostic output (no auto-dismiss).
	pub struct diagnostic_output;
	impl diagnostic_output {
		pub fn call(text: impl Into<String>) -> Notification {
			Notification::new(&NOTIF_DIAGNOSTIC_OUTPUT, text)
		}
	}

	/// Multi-line diagnostic warning (no auto-dismiss).
	pub struct diagnostic_warning;
	impl diagnostic_warning {
		pub fn call(text: impl Into<String>) -> Notification {
			Notification::new(&NOTIF_DIAGNOSTIC_WARNING, text)
		}
	}

	/// Parameterized notification for pending input prompts.
	pub struct pending_prompt;
	impl pending_prompt {
		pub fn call(prompt: &str) -> Notification {
			Notification::new(&NOTIF_PENDING_PROMPT, prompt.to_string())
		}
	}

	/// Parameterized notification for numeric count display.
	pub struct count_display;
	impl count_display {
		pub fn call(count: usize) -> Notification {
			Notification::new(&NOTIF_COUNT_DISPLAY, count.to_string())
		}
	}

	/// Parameterized notification: "Setting model to: X".
	pub struct acp_model_set;
	impl acp_model_set {
		pub fn call(model: &str) -> Notification {
			Notification::new(&NOTIF_ACP_MODEL_SET, format!("Setting model to: {}", model))
		}
	}

	/// ACP model info display (no auto-dismiss).
	pub struct acp_model_info;
	impl acp_model_info {
		pub fn call(text: impl Into<String>) -> Notification {
			Notification::new(&NOTIF_ACP_MODEL_INFO, text)
		}
	}

	/// Generic info notification (prefer typed keys when possible).
	pub struct info;
	impl info {
		pub fn call(msg: impl Into<String>) -> Notification {
			Notification::new(&NOTIF_INFO, msg)
		}
	}

	/// Generic warning notification (prefer typed keys when possible).
	pub struct warn;
	impl warn {
		pub fn call(msg: impl Into<String>) -> Notification {
			Notification::new(&NOTIF_WARN, msg)
		}
	}

	/// Generic error notification (prefer typed keys when possible).
	pub struct error;
	impl error {
		pub fn call(msg: impl Into<String>) -> Notification {
			Notification::new(&NOTIF_ERROR, msg)
		}
	}

	/// Generic success notification (prefer typed keys when possible).
	pub struct success;
	impl success {
		pub fn call(msg: impl Into<String>) -> Notification {
			Notification::new(&NOTIF_SUCCESS, msg)
		}
	}

	/// Generic debug notification (prefer typed keys when possible).
	pub struct debug;
	impl debug {
		pub fn call(msg: impl Into<String>) -> Notification {
			Notification::new(&NOTIF_DEBUG, msg)
		}
	}

	/// Debug notification for unhandled action results.
	pub struct unhandled_result;
	impl unhandled_result {
		pub fn call(discriminant: impl core::fmt::Debug) -> Notification {
			Notification::new(&NOTIF_UNHANDLED_RESULT, format!("Unhandled action result: {:?}", discriminant))
		}
	}
}
