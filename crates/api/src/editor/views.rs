//! Buffer and terminal view access.
//!
//! Provides convenient methods for accessing the focused view and navigating
//! between buffers and terminals.

use tome_manifest::SplitBuffer;

use crate::buffer::{Buffer, BufferId, BufferView, TerminalId};
use crate::terminal::TerminalBuffer;

use super::Editor;

impl Editor {
	/// Returns a reference to the currently focused text buffer.
	///
	/// Panics if the focused view is a terminal.
	#[inline]
	pub fn buffer(&self) -> &Buffer {
		match self.focused_view {
			BufferView::Text(id) => self.buffers.get(&id).expect("focused buffer must exist"),
			BufferView::Terminal(_) => panic!("focused view is a terminal, not a text buffer"),
		}
	}

	/// Returns a mutable reference to the currently focused text buffer.
	///
	/// Panics if the focused view is a terminal.
	#[inline]
	pub fn buffer_mut(&mut self) -> &mut Buffer {
		match self.focused_view {
			BufferView::Text(id) => self
				.buffers
				.get_mut(&id)
				.expect("focused buffer must exist"),
			BufferView::Terminal(_) => panic!("focused view is a terminal, not a text buffer"),
		}
	}

	/// Returns the currently focused view.
	pub fn focused_view(&self) -> BufferView {
		self.focused_view
	}

	/// Returns true if the focused view is a text buffer.
	pub fn is_text_focused(&self) -> bool {
		self.focused_view.is_text()
	}

	/// Returns true if the focused view is a terminal.
	pub fn is_terminal_focused(&self) -> bool {
		self.focused_view.is_terminal()
	}

	/// Returns the ID of the focused text buffer, if one is focused.
	pub fn focused_buffer_id(&self) -> Option<BufferId> {
		self.focused_view.as_text()
	}

	/// Returns the ID of the focused terminal, if one is focused.
	pub fn focused_terminal_id(&self) -> Option<TerminalId> {
		self.focused_view.as_terminal()
	}

	/// Returns all text buffer IDs.
	pub fn buffer_ids(&self) -> Vec<BufferId> {
		self.buffers.keys().copied().collect()
	}

	/// Returns all terminal IDs.
	pub fn terminal_ids(&self) -> Vec<TerminalId> {
		self.terminals.keys().copied().collect()
	}

	/// Returns a reference to a specific buffer by ID.
	pub fn get_buffer(&self, id: BufferId) -> Option<&Buffer> {
		self.buffers.get(&id)
	}

	/// Returns a mutable reference to a specific buffer by ID.
	pub fn get_buffer_mut(&mut self, id: BufferId) -> Option<&mut Buffer> {
		self.buffers.get_mut(&id)
	}

	/// Returns a reference to a specific terminal by ID.
	pub fn get_terminal(&self, id: TerminalId) -> Option<&TerminalBuffer> {
		self.terminals.get(&id)
	}

	/// Returns a mutable reference to a specific terminal by ID.
	pub fn get_terminal_mut(&mut self, id: TerminalId) -> Option<&mut TerminalBuffer> {
		self.terminals.get_mut(&id)
	}

	/// Returns the number of open text buffers.
	pub fn buffer_count(&self) -> usize {
		self.buffers.len()
	}

	/// Returns the number of open terminals.
	pub fn terminal_count(&self) -> usize {
		self.terminals.len()
	}

	/// Returns the cursor style for the focused terminal, if any.
	pub fn focused_terminal_cursor_style(&self) -> Option<tome_manifest::SplitCursorStyle> {
		let terminal_id = self.focused_terminal_id()?;
		let terminal = self.get_terminal(terminal_id)?;
		terminal.cursor().map(|c| c.style)
	}
}
