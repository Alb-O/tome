//! Inlay hints management for the editor.
//!
//! This module provides methods for caching and refreshing LSP inlay hints.
//! Inlay hints are virtual text shown inline (type annotations, parameter names).

use crate::buffer::BufferId;
use crate::editor::Editor;
use crate::render::{PreparedInlayHints, prepare_inlay_hints};

/// Inlay hints refresh state for a buffer.
#[derive(Debug, Clone, Default)]
pub struct InlayHintsState {
	/// Last viewport start line when hints were fetched.
	pub last_start_line: usize,
	/// Last viewport end line when hints were fetched.
	pub last_end_line: usize,
	/// Last document version when hints were fetched.
	pub last_version: u64,
}

impl Editor {
	/// Clears the inlay hints cache for a buffer.
	///
	/// Called when buffer content changes to invalidate stale hints.
	pub fn clear_inlay_hints(&mut self, buffer_id: BufferId) {
		self.inlay_hints_cache.remove(&buffer_id);
	}

	/// Clears all inlay hints caches.
	pub fn clear_all_inlay_hints(&mut self) {
		self.inlay_hints_cache.clear();
	}

	/// Stores prepared inlay hints for a buffer.
	pub fn set_inlay_hints(&mut self, buffer_id: BufferId, hints: PreparedInlayHints) {
		self.inlay_hints_cache.insert(buffer_id, hints);
		self.needs_redraw = true;
	}

	/// Returns whether inlay hints are enabled for a buffer.
	pub fn inlay_hints_enabled(&self, buffer_id: BufferId) -> bool {
		use xeno_registry::options::keys;

		// Check if option is enabled (buffer-level resolution)
		if let Some(buffer) = self.buffers.get_buffer(buffer_id) {
			buffer.option(keys::INLAY_HINTS_ENABLED, self)
		} else {
			true // Default to enabled
		}
	}

	/// Prepares inlay hints from raw LSP hints for a buffer.
	///
	/// Converts LSP InlayHint objects to display-ready format and stores in cache.
	pub fn prepare_inlay_hints_for_buffer(
		&mut self,
		buffer_id: BufferId,
		hints: Vec<xeno_lsp::lsp_types::InlayHint>,
	) {
		let Some(buffer) = self.buffers.get_buffer(buffer_id) else {
			return;
		};

		// Use UTF-16 encoding by default (common LSP default)
		let encoding = xeno_lsp::OffsetEncoding::Utf16;
		let prepared = prepare_inlay_hints(&buffer.doc().content, &hints, encoding);
		self.set_inlay_hints(buffer_id, prepared);
	}

	/// Requests inlay hints refresh for visible buffers.
	///
	/// This is an async operation - hints will be updated via callback when available.
	/// Returns a list of (buffer_id, start_line, end_line) for buffers that need refresh.
	pub fn get_buffers_needing_inlay_hints(&self) -> Vec<(BufferId, usize, usize)> {
		let mut result = Vec::new();

		// Get all visible buffer views
		for buffer_id in self.windows.base_window().layout.views() {

			// Check if inlay hints are enabled for this buffer
			if !self.inlay_hints_enabled(buffer_id) {
				continue;
			}

			// Get buffer
			let Some(buffer) = self.buffers.get_buffer(buffer_id) else {
				continue;
			};

			// Check if buffer has a path (LSP requires file path)
			if buffer.path().is_none() {
				continue;
			}

			// Get visible range
			let start_line = buffer.scroll_line;
			let end_line = start_line + 50; // Rough viewport height estimate

			// Check if we need to refresh (no cache or viewport changed significantly)
			let needs_refresh = !self.inlay_hints_cache.contains_key(&buffer_id);

			if needs_refresh {
				result.push((buffer_id, start_line, end_line));
			}
		}

		result
	}
}
