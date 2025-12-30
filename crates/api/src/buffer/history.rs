//! Undo/redo history for buffers.
//!
//! Note: Most undo/redo operations are now handled at the Editor level
//! (see `crates/api/src/editor/history.rs`) to properly sync selections
//! across sibling buffers. The methods here are retained for compatibility
//! but delegate to single-buffer behavior.

/// Result of an undo/redo operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryResult {
	/// Operation succeeded.
	Success,
	/// Nothing to undo.
	NothingToUndo,
	/// Nothing to redo.
	NothingToRedo,
}
