//! Editor-level navigation operations.
//!
//! Most navigation is delegated to Buffer. This module provides
//! Editor-specific wrappers where needed.

#[cfg(feature = "lsp")]
use std::sync::Arc;

use xeno_base::ScrollDirection;
use xeno_base::range::Direction as MoveDir;

use super::Editor;

impl Editor {
	/// Returns the line number containing the cursor.
	pub fn cursor_line(&self) -> usize {
		self.buffer().cursor_line()
	}

	/// Returns the column of the cursor within its line.
	pub fn cursor_col(&self) -> usize {
		self.buffer().cursor_col()
	}

	/// Computes the gutter width based on total line count.
	pub fn gutter_width(&self) -> u16 {
		self.buffer().gutter_width()
	}

	/// Moves cursors vertically, accounting for line wrapping.
	///
	/// Resolves the `tab-width` option and delegates to Buffer.
	pub fn move_visual_vertical(&mut self, direction: MoveDir, count: usize, extend: bool) {
		let tab_width = self.tab_width();
		self.buffer_mut()
			.move_visual_vertical(direction, count, extend, tab_width);
	}

	/// Handles mouse scroll events.
	///
	/// Resolves the `tab-width` option and delegates to Buffer.
	pub(crate) fn handle_mouse_scroll(&mut self, direction: ScrollDirection, count: usize) {
		let tab_width = self.tab_width();
		self.buffer_mut().handle_mouse_scroll(direction, count, tab_width);
	}

	/// Navigate to the next or previous diagnostic.
	///
	/// Returns the diagnostic message if found, None otherwise.
	#[cfg(feature = "lsp")]
	pub(crate) fn navigate_diagnostic(&mut self, forward: bool) -> Option<String> {
		use crate::render::prepare_diagnostics;

		type LspManager = Arc<crate::lsp::LspManager>;

		// Get diagnostic target position (separate scope to release borrow)
		let target_info = {
			let buffer = self.buffer();
			let cursor = buffer.cursor;

			// Get diagnostics from LSP
			let lsp = self.extensions.get::<LspManager>()?;
			let diagnostics = lsp.get_diagnostics(buffer);
			if diagnostics.is_empty() {
				return None;
			}

			// Prepare diagnostics (converts to char positions)
			let encoding = xeno_lsp::OffsetEncoding::Utf16;
			let prepared = prepare_diagnostics(&buffer.doc().content, &diagnostics, encoding);

			// Find next/prev diagnostic relative to cursor
			let target = if forward {
				// Find first diagnostic after cursor (or wrap to first)
				prepared.all.iter()
					.find(|d| d.char_start > cursor)
					.or_else(|| prepared.all.first())
			} else {
				// Find last diagnostic before cursor (or wrap to last)
				prepared.all.iter().rev()
					.find(|d| d.char_start < cursor)
					.or_else(|| prepared.all.last())
			}?;

			(target.char_start, target.message.clone())
		};

		let (char_start, message) = target_info;

		// Move cursor to diagnostic start
		self.buffer_mut().cursor = char_start;
		self.buffer_mut().selection = xeno_base::Selection::point(char_start);

		Some(message)
	}
}
