//! Text editing operations.
//!
//! Insert, delete, yank, paste, and transaction application.

use xeno_base::Transaction;
#[cfg(feature = "lsp")]
use xeno_base::{range::CharIdx, Selection};
use xeno_registry_notifications::keys;

use super::Editor;

impl Editor {
	pub(crate) fn guard_readonly(&mut self) -> bool {
		if self.buffer().is_readonly() {
			self.notify(keys::buffer_readonly);
			return false;
		}
		true
	}

	/// Inserts text at the current cursor position(s).
	pub fn insert_text(&mut self, text: &str) {
		let buffer_id = self.focused_view();

		if !self.guard_readonly() {
			return;
		}

		if self.buffer().mode() == xeno_base::Mode::Insert {
			self.save_insert_undo_state();
		} else {
			self.save_undo_state();
		}

		// Prepare the transaction and new selection (without applying)
		let (tx, new_selection) = {
			let buffer = self
				.buffers
				.get_buffer_mut(buffer_id)
				.expect("focused buffer must exist");
			buffer.prepare_insert(text)
		};

		let applied = {
			let buffer = self
				.buffers
				.get_buffer_mut(buffer_id)
				.expect("focused buffer must exist");
			let applied = buffer.apply_transaction_with_syntax(&tx, &self.language_loader);
			if applied {
				buffer.finalize_selection(new_selection);
			}
			applied
		};

		if !applied {
			self.notify(keys::buffer_readonly);
			return;
		}

		self.sync_sibling_selections(&tx);
		self.dirty_buffers.insert(buffer_id);
	}

	/// Copies the current selection to the yank register.
	pub fn yank_selection(&mut self) {
		if let Some((text, count)) = self.buffer_mut().yank_selection() {
			self.registers.yank = text;
			self.notify(keys::yanked_chars::call(count));
		}
	}

	/// Pastes the yank register content after the cursor.
	pub fn paste_after(&mut self) {
		if self.registers.yank.is_empty() {
			return;
		}

		if !self.guard_readonly() {
			return;
		}

		let buffer_id = self.focused_view();

		self.save_undo_state();
		let yank = self.registers.yank.clone();

		// Prepare the transaction and new selection (without applying)
		let Some((tx, new_selection)) = ({
			let buffer = self
				.buffers
				.get_buffer_mut(buffer_id)
				.expect("focused buffer must exist");
			buffer.prepare_paste_after(&yank)
		}) else {
			return;
		};

		let applied = {
			let buffer = self
				.buffers
				.get_buffer_mut(buffer_id)
				.expect("focused buffer must exist");
			let applied = buffer.apply_transaction_with_syntax(&tx, &self.language_loader);
			if applied {
				buffer.finalize_selection(new_selection);
			}
			applied
		};

		if !applied {
			self.notify(keys::buffer_readonly);
			return;
		}

		self.sync_sibling_selections(&tx);
		self.dirty_buffers.insert(buffer_id);
	}

	/// Pastes the yank register content before the cursor.
	pub fn paste_before(&mut self) {
		if self.registers.yank.is_empty() {
			return;
		}

		if !self.guard_readonly() {
			return;
		}

		let buffer_id = self.focused_view();

		self.save_undo_state();
		let yank = self.registers.yank.clone();

		// Prepare the transaction and new selection (without applying)
		let Some((tx, new_selection)) = ({
			let buffer = self
				.buffers
				.get_buffer_mut(buffer_id)
				.expect("focused buffer must exist");
			buffer.prepare_paste_before(&yank)
		}) else {
			return;
		};

		let applied = {
			let buffer = self
				.buffers
				.get_buffer_mut(buffer_id)
				.expect("focused buffer must exist");
			let applied = buffer.apply_transaction_with_syntax(&tx, &self.language_loader);
			if applied {
				buffer.finalize_selection(new_selection);
			}
			applied
		};

		if !applied {
			self.notify(keys::buffer_readonly);
			return;
		}

		self.sync_sibling_selections(&tx);
		self.dirty_buffers.insert(buffer_id);
	}

	/// Deletes the currently selected text.
	pub fn delete_selection(&mut self) {
		if self.buffer().selection.primary().is_empty() {
			return;
		}

		if !self.guard_readonly() {
			return;
		}

		let buffer_id = self.focused_view();

		self.save_undo_state();

		// Prepare the transaction and new selection (without applying)
		let Some((tx, new_selection)) = ({
			let buffer = self
				.buffers
				.get_buffer_mut(buffer_id)
				.expect("focused buffer must exist");
			buffer.prepare_delete_selection()
		}) else {
			return;
		};

		let applied = {
			let buffer = self
				.buffers
				.get_buffer_mut(buffer_id)
				.expect("focused buffer must exist");
			let applied = buffer.apply_transaction_with_syntax(&tx, &self.language_loader);
			if applied {
				buffer.finalize_selection(new_selection);
			}
			applied
		};

		if !applied {
			self.notify(keys::buffer_readonly);
			return;
		}

		self.sync_sibling_selections(&tx);
		self.dirty_buffers.insert(buffer_id);
	}

	/// Applies a transaction to the focused buffer.
	pub fn apply_transaction(&mut self, tx: &Transaction) {
		let buffer_id = self.focused_view();
		let applied = self
			.buffers
			.get_buffer_mut(buffer_id)
			.expect("focused buffer must exist")
			.apply_transaction_with_syntax(tx, &self.language_loader);
		if !applied {
			self.notify(keys::buffer_readonly);
			return;
		}
		self.dirty_buffers.insert(buffer_id);
		self.sync_sibling_selections(tx);
	}

	/// Triggers a full syntax reparse of the focused buffer.
	pub fn reparse_syntax(&mut self) {
		let buffer_id = self.focused_view();

		// Access buffer directly to avoid borrow conflict with language_loader.
		let buffer = self
			.buffers
			.get_buffer_mut(buffer_id)
			.expect("focused buffer must exist");
		buffer.reparse_syntax(&self.language_loader);
	}

	/// Updates the completion filter after a character is inserted.
	///
	/// If a completion popup is active, this method:
	/// 1. Gets the current text from trigger_column to cursor
	/// 2. Updates the completion popup's filter
	/// 3. Dismisses the popup if no matches remain
	#[cfg(feature = "lsp")]
	pub fn update_completion_after_insert(&mut self) {
		// Check if completion popup is active
		if !self.ui.has_completion_popup() {
			return;
		}

		// Get the filter text from trigger_column to cursor
		let filter_text = self.get_word_before_cursor();

		// Update the completion popup's filter via the popup manager
		// We need to access the popup and call set_filter on it
		self.ui.update_completion_filter(filter_text);

		// Check if completion should be dismissed (no matches)
		if self.ui.should_dismiss_completion() {
			self.ui.dismiss_popup("lsp-completion");
		}
	}

	/// Attempts to accept a completion from the UI popup.
	///
	/// If a completion popup is active and Tab/Enter is pressed, this method:
	/// 1. Gets the acceptance result from the completion popup
	/// 2. Deletes text from trigger_column to cursor
	/// 3. Inserts the completion text
	///
	/// Returns `true` if completion was accepted and applied, `false` otherwise.
	#[cfg(feature = "lsp")]
	pub fn try_accept_completion(&mut self, key: &termina::event::KeyEvent) -> bool {
		// Try to get the acceptance result from the popup
		let Some((accept_result, trigger_column)) = self.ui.try_accept_completion(key) else {
			return false;
		};

		if !self.guard_readonly() {
			return false;
		}

		let buffer_id = self.focused_view();
		self.save_undo_state();

		// Get the line start and compute absolute positions
		let (line_start, current_pos) = {
			let buffer = self.buffer();
			let cursor = buffer.cursor;
			let rope = &buffer.doc().content;

			// Get the line the cursor is on
			let line_idx = rope.char_to_line(cursor);
			let line_start = rope.line_to_char(line_idx);

			(line_start, cursor)
		};

		// Compute the range to replace: from trigger_column to current cursor
		let start_pos = line_start + trigger_column;
		let end_pos = current_pos;

		// If start > end (shouldn't happen, but guard against it), bail
		if start_pos > end_pos {
			return false;
		}

		// Create a selection spanning the range to replace
		let replace_selection = Selection::single(start_pos, end_pos);

		// First, delete the selection range, then insert the completion text
		// We'll use a transaction approach
		{
			let buffer = self
				.buffers
				.get_buffer_mut(buffer_id)
				.expect("focused buffer must exist");

			// Save the original selection and set the replacement selection
			let original_selection = buffer.selection.clone();
			buffer.selection = replace_selection;

			// Delete the selection
			if !buffer.selection.primary().is_empty() {
				let delete_result = buffer.prepare_delete_selection();
				if let Some((tx, _new_selection)) = delete_result {
					buffer.apply_transaction_with_syntax(&tx, &self.language_loader);
				}
			}

			// Now insert the completion text at the current position
			let (tx, new_selection) = buffer.prepare_insert(&accept_result.insert_text);
			let applied = buffer.apply_transaction_with_syntax(&tx, &self.language_loader);
			if applied {
				buffer.finalize_selection(new_selection);
			}

			// Handle additional text edits (auto-imports) if any
			// TODO: Apply additional_edits from accept_result.additional_edits
			// This requires converting LSP ranges to buffer positions
		}

		self.dirty_buffers.insert(buffer_id);
		true
	}

	/// Attempts to accept a location from the location picker popup.
	///
	/// If a location picker popup is active and Enter is pressed, this method:
	/// 1. Gets the selected location from the popup
	/// 2. Navigates to that location
	///
	/// Returns `true` if a location was accepted and navigated to, `false` otherwise.
	#[cfg(feature = "lsp")]
	pub async fn try_accept_location_picker(&mut self, key: &termina::event::KeyEvent) -> bool {
		// Try to get the selected location from the popup
		let Some(location) = self.ui.try_accept_location_picker(key) else {
			return false;
		};

		// Navigate to the location
		crate::lsp_ui::navigate_to_location(self, &location).await
	}
}
