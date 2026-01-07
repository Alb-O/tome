//! Undo/redo history types.

use std::collections::HashMap;

use xeno_base::{Rope, Selection};

use crate::buffer::BufferId;

/// Undo/redo history entry storing document state and per-view selections.
#[derive(Clone)]
pub struct HistoryEntry {
	/// Document content at this point in history.
	pub doc: Rope,
	/// Per-buffer selections at this point in history.
	pub selections: HashMap<BufferId, Selection>,
}
