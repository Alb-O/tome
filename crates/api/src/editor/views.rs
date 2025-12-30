//! Buffer and panel view access.
//!
//! Provides convenient methods for accessing the focused view and navigating
//! between buffers and panels. These delegate to [`BufferManager`] and [`PanelRegistry`].

use evildoer_manifest::{PanelId, SplitBuffer};

use super::Editor;
use crate::buffer::{Buffer, BufferId, BufferView};
use crate::terminal::TerminalBuffer;

impl Editor {
	/// Returns a reference to the currently focused text buffer.
	///
	/// Panics if the focused view is not a text buffer.
	#[inline]
	pub fn buffer(&self) -> &Buffer {
		self.buffers.focused_buffer()
	}

	/// Returns a mutable reference to the currently focused text buffer.
	///
	/// Panics if the focused view is not a text buffer.
	#[inline]
	pub fn buffer_mut(&mut self) -> &mut Buffer {
		self.buffers.focused_buffer_mut()
	}

	/// Returns the currently focused view.
	pub fn focused_view(&self) -> BufferView {
		self.buffers.focused_view()
	}

	/// Returns true if the focused view is a text buffer.
	pub fn is_text_focused(&self) -> bool {
		self.buffers.is_text_focused()
	}

	/// Returns true if the focused view is a panel.
	pub fn is_panel_focused(&self) -> bool {
		matches!(self.focused_view(), BufferView::Panel(_))
	}

	/// Returns true if the focused view is a terminal panel.
	pub fn is_terminal_focused(&self) -> bool {
		self.focused_view()
			.as_panel()
			.is_some_and(|id| self.panels.get::<TerminalBuffer>(id).is_some())
	}

	/// Returns true if the focused view is a debug panel.
	pub fn is_debug_focused(&self) -> bool {
		self.focused_view()
			.as_panel()
			.is_some_and(|id| self.panels.get::<crate::debug::DebugPanel>(id).is_some())
	}

	/// Returns the ID of the focused text buffer, if one is focused.
	pub fn focused_buffer_id(&self) -> Option<BufferId> {
		self.buffers.focused_buffer_id()
	}

	/// Returns the ID of the focused panel, if one is focused.
	pub fn focused_panel_id(&self) -> Option<PanelId> {
		self.focused_view().as_panel()
	}

	/// Returns all text buffer IDs.
	pub fn buffer_ids(&self) -> Vec<BufferId> {
		self.buffers.buffer_ids().collect()
	}

	/// Returns a reference to a specific buffer by ID.
	pub fn get_buffer(&self, id: BufferId) -> Option<&Buffer> {
		self.buffers.get_buffer(id)
	}

	/// Returns a mutable reference to a specific buffer by ID.
	pub fn get_buffer_mut(&mut self, id: BufferId) -> Option<&mut Buffer> {
		self.buffers.get_buffer_mut(id)
	}

	/// Returns a reference to a terminal panel by ID.
	pub fn get_terminal(&self, id: PanelId) -> Option<&TerminalBuffer> {
		self.panels.get::<TerminalBuffer>(id)
	}

	/// Returns a mutable reference to a terminal panel by ID.
	pub fn get_terminal_mut(&mut self, id: PanelId) -> Option<&mut TerminalBuffer> {
		self.panels.get_mut::<TerminalBuffer>(id)
	}

	/// Returns the number of open text buffers.
	pub fn buffer_count(&self) -> usize {
		self.buffers.buffer_count()
	}

	/// Returns the cursor style for the focused terminal panel, if any.
	pub fn focused_terminal_cursor_style(&self) -> Option<evildoer_manifest::SplitCursorStyle> {
		let panel_id = self.focused_panel_id()?;
		let terminal = self.get_terminal(panel_id)?;
		terminal.cursor().map(|c| c.style)
	}
}
