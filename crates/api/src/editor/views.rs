//! Buffer access and viewport management.
//!
//! Provides convenient methods for accessing buffers. Delegates to [`BufferManager`].

use super::Editor;
use crate::buffer::{Buffer, BufferId, BufferView};

impl Editor {
	/// Returns a reference to the currently focused text buffer.
	#[inline]
	pub fn buffer(&self) -> &Buffer {
		self.buffers.focused_buffer()
	}

	/// Returns a mutable reference to the currently focused text buffer.
	#[inline]
	pub fn buffer_mut(&mut self) -> &mut Buffer {
		self.buffers.focused_buffer_mut()
	}

	/// Returns the currently focused view (buffer ID).
	pub fn focused_view(&self) -> BufferView {
		self.buffers.focused_view()
	}

	/// Returns true if the focused view is a text buffer.
	///
	/// Always returns true since all views are now text buffers.
	pub fn is_text_focused(&self) -> bool {
		true
	}

	/// Returns the ID of the focused text buffer.
	pub fn focused_buffer_id(&self) -> Option<BufferId> {
		Some(self.buffers.focused_view())
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

	/// Returns the number of open text buffers.
	pub fn buffer_count(&self) -> usize {
		self.buffers.buffer_count()
	}
}
