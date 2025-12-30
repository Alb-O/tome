//! Editor-level undo/redo operations.
//!
//! Undo/redo is handled at the Editor level to properly sync selections
//! across all buffers viewing the same document.

use std::collections::HashMap;

use evildoer_base::Selection;

use crate::buffer::{BufferId, BufferView};
use crate::editor::Editor;

impl Editor {
	/// Collects selections from all buffers sharing a document.
	fn collect_sibling_selections(&self, doc_id: crate::buffer::DocumentId) -> HashMap<BufferId, Selection> {
		self.buffers
			.buffers()
			.filter(|b| b.document_id() == doc_id)
			.map(|b| (b.id, b.selection.clone()))
			.collect()
	}

	/// Saves current state to undo history.
	///
	/// Collects selections from all buffers viewing the same document.
	pub fn save_undo_state(&mut self) {
		let BufferView::Text(buffer_id) = self.buffers.focused_view() else {
			return;
		};

		let doc_id = self
			.buffers
			.get_buffer(buffer_id)
			.expect("focused buffer must exist")
			.document_id();

		let selections = self.collect_sibling_selections(doc_id);

		let buffer = self
			.buffers
			.get_buffer_mut(buffer_id)
			.expect("focused buffer must exist");
		buffer.doc_mut().save_undo_state(selections, buffer_id);
	}

	/// Saves undo state for insert mode, grouping consecutive inserts.
	///
	/// Collects selections from all buffers viewing the same document.
	pub(crate) fn save_insert_undo_state(&mut self) {
		let BufferView::Text(buffer_id) = self.buffers.focused_view() else {
			return;
		};

		let doc_id = self
			.buffers
			.get_buffer(buffer_id)
			.expect("focused buffer must exist")
			.document_id();

		let selections = self.collect_sibling_selections(doc_id);

		let buffer = self
			.buffers
			.get_buffer_mut(buffer_id)
			.expect("focused buffer must exist");
		buffer.doc_mut().save_insert_undo_state(selections, buffer_id);
	}

	/// Undoes the last change and restores selections for all sibling buffers.
	pub fn undo(&mut self) {
		let BufferView::Text(buffer_id) = self.buffers.focused_view() else {
			self.notify("warn", "Cannot undo in terminal");
			return;
		};

		let doc_id = self
			.buffers
			.get_buffer(buffer_id)
			.expect("focused buffer must exist")
			.document_id();

		let current_selections = self.collect_sibling_selections(doc_id);

		// Perform the undo
		let restored_selections = {
			let buffer = self
				.buffers
				.get_buffer_mut(buffer_id)
				.expect("focused buffer must exist");
			buffer.doc_mut().undo(current_selections, buffer_id, &self.language_loader)
		};

		let Some(selections) = restored_selections else {
			self.notify("warn", "Nothing to undo");
			return;
		};

		// Restore selections to all buffers that were saved
		for buffer in self.buffers.buffers_mut() {
			if buffer.document_id() == doc_id {
				if let Some(selection) = selections.get(&buffer.id) {
					buffer.selection = selection.clone();
					buffer.cursor = buffer.selection.primary().head;
				}
				buffer.ensure_valid_selection();
			}
		}

		self.notify("info", "Undo");
	}

	/// Redoes the last undone change and restores selections for all sibling buffers.
	pub fn redo(&mut self) {
		let BufferView::Text(buffer_id) = self.buffers.focused_view() else {
			self.notify("warn", "Cannot redo in terminal");
			return;
		};

		let doc_id = self
			.buffers
			.get_buffer(buffer_id)
			.expect("focused buffer must exist")
			.document_id();

		let current_selections = self.collect_sibling_selections(doc_id);

		// Perform the redo
		let restored_selections = {
			let buffer = self
				.buffers
				.get_buffer_mut(buffer_id)
				.expect("focused buffer must exist");
			buffer.doc_mut().redo(current_selections, buffer_id, &self.language_loader)
		};

		let Some(selections) = restored_selections else {
			self.notify("warn", "Nothing to redo");
			return;
		};

		// Restore selections to all buffers that were saved
		for buffer in self.buffers.buffers_mut() {
			if buffer.document_id() == doc_id {
				if let Some(selection) = selections.get(&buffer.id) {
					buffer.selection = selection.clone();
					buffer.cursor = buffer.selection.primary().head;
				}
				buffer.ensure_valid_selection();
			}
		}

		self.notify("info", "Redo");
	}
}
