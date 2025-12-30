//! Undo/redo result types.

/// Result of an undo/redo operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryResult {
	Success,
	NothingToUndo,
	NothingToRedo,
}
