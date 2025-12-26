use tome_base::{Rope, Selection};
use tome_manifest::CompletionItem;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MessageKind {
	Info,
	Warning,
	Error,
}

#[derive(Clone, Debug)]
pub struct Message {
	pub text: String,
	pub kind: MessageKind,
}

/// A history entry for undo/redo.
#[derive(Clone)]
pub struct HistoryEntry {
	pub doc: Rope,
	pub selection: Selection,
}

#[derive(Default)]
pub struct Registers {
	pub yank: String,
}

#[derive(Clone, Default)]
pub struct CompletionState {
	pub items: Vec<CompletionItem>,
	pub selected_idx: Option<usize>,
	pub active: bool,
	/// Start position in the input where replacement begins.
	/// When a completion is accepted, text from this position to cursor is replaced.
	pub replace_start: usize,
}
