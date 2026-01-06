//! View focus management.
//!
//! Focusing buffers and navigating between views.

use xeno_base::Mode;
use xeno_registry::{HookContext, HookEventData, ViewId, emit_sync_with as emit_hook_sync_with};

use super::Editor;
use crate::buffer::{BufferId, BufferView, Direction};
use crate::window::{Window, WindowId};

/// Panel identifier used by focus targets.
pub type PanelId = String;

/// Identifies what has keyboard focus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FocusTarget {
	Buffer { window: WindowId, buffer: BufferId },
	Panel(PanelId),
}

/// Reason for focus change (for hooks and debugging).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusReason {
	/// User clicked on target.
	Click,
	/// User used keybinding (e.g., Ctrl+P for command palette).
	Keybinding,
	/// Programmatic focus (e.g., opening a new window).
	Programmatic,
	/// Mouse hover (if focus-follows-mouse enabled).
	Hover,
}

/// Converts a buffer view to a hook-compatible view ID.
fn hook_view_id(view: BufferView) -> ViewId {
	ViewId::text(view.0)
}

impl Editor {
	/// Focuses a specific view explicitly (user action like click or keybinding).
	///
	/// Returns true if the view exists and was focused.
	/// Explicit focus can override sticky focus and will close dockables.
	pub fn focus_view(&mut self, view: BufferView) -> bool {
		let window_id = self.windows.base_id();
		self.focus_buffer_in_window(window_id, view, true)
	}

	/// Focuses a specific view implicitly (mouse hover).
	///
	/// Returns true if the view exists and was focused.
	/// Respects sticky focus - won't steal focus from sticky views.
	pub fn focus_view_implicit(&mut self, view: BufferView) -> bool {
		let current = self.focused_view();
		if current == view || self.sticky_views.contains(&current) {
			return false;
		}
		let window_id = self.windows.base_id();
		self.focus_buffer_in_window(window_id, view, false)
	}

	/// Internal focus implementation, handling sticky views.
	pub(super) fn focus_buffer_in_window(
		&mut self,
		window_id: WindowId,
		view: BufferView,
		explicit: bool,
	) -> bool {
		if self.buffers.get_buffer(view).is_none() {
			return false;
		}

		let old_focus = self.focus.clone();
		let old_view = self.focused_view();
		let base_window_id = self.windows.base_id();

		self.focus = FocusTarget::Buffer {
			window: window_id,
			buffer: view,
		};
		if window_id == base_window_id {
			self.base_window_mut().focused_buffer = view;
		}
		let _ = self.buffers.set_focused_view(view);
		self.needs_redraw = true;

		if explicit && view != old_view {
			self.sticky_views.remove(&old_view);
		}

		if view != old_view {
			emit_hook_sync_with(
				&HookContext::new(
					HookEventData::ViewFocusChanged {
						view_id: hook_view_id(view),
						prev_view_id: Some(hook_view_id(old_view)),
					},
					Some(&self.extensions),
				),
				&mut self.hook_runtime,
			);
		}

		let new_focus = self.focus.clone();
		self.handle_window_focus_change(old_focus, &new_focus);

		true
	}

	/// Focuses a specific buffer by ID.
	///
	/// Returns true if the buffer exists and was focused.
	pub fn focus_buffer(&mut self, id: BufferId) -> bool {
		self.focus_view(id)
	}

	/// Focuses the next view in the layout.
	pub fn focus_next_view(&mut self) {
		let next = self
			.layout
			.next_view(&self.base_window().layout, self.focused_view());
		self.focus_view(next);
	}

	/// Focuses the previous view in the layout.
	pub fn focus_prev_view(&mut self) {
		let prev = self
			.layout
			.prev_view(&self.base_window().layout, self.focused_view());
		self.focus_view(prev);
	}

	/// Focuses the next text buffer in the layout.
	pub fn focus_next_buffer(&mut self) {
		let current_id = self.focused_view();
		let next_id = self
			.layout
			.next_buffer(&self.base_window().layout, current_id);
		self.focus_buffer(next_id);
	}

	/// Focuses the previous text buffer in the layout.
	pub fn focus_prev_buffer(&mut self) {
		let current_id = self.focused_view();
		let prev_id = self
			.layout
			.prev_buffer(&self.base_window().layout, current_id);
		self.focus_buffer(prev_id);
	}

	/// Focuses the view in the given direction, using cursor position as tiebreaker.
	pub fn focus_direction(&mut self, direction: Direction) {
		let area = self.doc_area();
		let current = self.focused_view();
		let hint = self.cursor_screen_pos(direction, area);

		if let Some(target) = self.layout.view_in_direction(
			&self.base_window().layout,
			area,
			current,
			direction,
			hint,
		) {
			self.focus_view(target);
		}
	}

	pub(super) fn sync_focus_from_ui(&mut self) {
		let old_focus = self.focus.clone();
		if let Some(panel_id) = self.ui.focused_panel_id() {
			self.focus = FocusTarget::Panel(panel_id.to_string());
		} else if matches!(self.focus, FocusTarget::Panel(_)) {
			let buffer = self.base_window().focused_buffer;
			self.focus = FocusTarget::Buffer {
				window: self.windows.base_id(),
				buffer,
			};
		}

		if old_focus != self.focus {
			let new_focus = self.focus.clone();
			self.handle_window_focus_change(old_focus, &new_focus);
		}
	}

	/// Returns cursor screen position along the perpendicular axis for directional hints.
	fn cursor_screen_pos(&self, direction: Direction, area: xeno_tui::layout::Rect) -> u16 {
		let buffer = self.buffer();
		let view_rect = self
			.layout
			.compute_view_areas(&self.base_window().layout, area)
			.into_iter()
			.find(|(v, _)| *v == self.focused_view())
			.map(|(_, r)| r)
			.unwrap_or(area);

		match direction {
			Direction::Left | Direction::Right => {
				let visible_line = buffer.cursor_line().saturating_sub(buffer.scroll_line);
				view_rect.y + (visible_line as u16).min(view_rect.height.saturating_sub(1))
			}
			Direction::Up | Direction::Down => {
				let gutter = buffer.gutter_width();
				view_rect.x
					+ gutter + (buffer.cursor_col() as u16)
					.min(view_rect.width.saturating_sub(gutter + 1))
			}
		}
	}

	/// Returns the cursor's screen position as (x, y) coordinates.
	///
	/// This computes where the cursor would appear on screen based on the
	/// current view area, scroll position, and cursor location in the buffer.
	///
	/// Returns `None` if the cursor is not visible in the current viewport.
	pub fn cursor_screen_position(&self) -> Option<(u16, u16)> {
		let area = self.doc_area();
		let buffer = self.buffer();
		let view_rect = self
			.layout
			.compute_view_areas(&self.base_window().layout, area)
			.into_iter()
			.find(|(v, _)| *v == self.focused_view())
			.map(|(_, r)| r)
			.unwrap_or(area);

		// Calculate Y position
		let cursor_line = buffer.cursor_line();
		let visible_line = cursor_line.saturating_sub(buffer.scroll_line);
		if visible_line >= view_rect.height as usize {
			return None; // Cursor is below visible area
		}
		let y = view_rect.y + visible_line as u16;

		// Calculate X position (no horizontal scrolling currently)
		let gutter = buffer.gutter_width();
		let cursor_col = buffer.cursor_col() as u16;
		let content_width = view_rect.width.saturating_sub(gutter);
		if cursor_col >= content_width {
			return None; // Cursor is to the right of visible area
		}
		let x = view_rect.x + gutter + cursor_col;

		Some((x, y))
	}

	/// Returns the column position where the current word starts.
	///
	/// This is used to determine the trigger column for completion requests.
	/// Words are delimited by whitespace and punctuation.
	pub fn get_word_start_column(&self) -> usize {
		let buffer = self.buffer();
		let cursor = buffer.cursor;
		let rope = &buffer.doc().content;
		
		// Find the start of the current line
		let line_idx = rope.char_to_line(cursor);
		let line_start = rope.line_to_char(line_idx);
		
		// Get the text from line start to cursor
		let line_to_cursor: String = rope.slice(line_start..cursor).chars().collect();
		
		// Find the start of the word by scanning backwards
		let mut word_start = line_to_cursor.len();
		for (i, c) in line_to_cursor.chars().rev().enumerate() {
			if !c.is_alphanumeric() && c != '_' {
				word_start = line_to_cursor.len() - i;
				break;
			}
			if i == line_to_cursor.len() - 1 {
				word_start = 0;
			}
		}
		
		word_start
	}

	/// Returns the text from the word start to the cursor position.
	///
	/// This is used as the initial filter text for completion.
	pub fn get_word_before_cursor(&self) -> String {
		let buffer = self.buffer();
		let cursor = buffer.cursor;
		let rope = &buffer.doc().content;
		
		// Find the start of the current line
		let line_idx = rope.char_to_line(cursor);
		let line_start = rope.line_to_char(line_idx);
		
		// Get the text from line start to cursor
		let line_to_cursor: String = rope.slice(line_start..cursor).chars().collect();
		
		// Find the start of the word
		let word_start = self.get_word_start_column();
		
		// Extract the word text
		if word_start < line_to_cursor.len() {
			line_to_cursor[word_start..].to_string()
		} else {
			String::new()
		}
	}

	/// Returns the current editing mode (Normal, Insert, Visual, etc.).
	pub fn mode(&self) -> Mode {
		self.buffer().input.mode()
	}

	/// Returns the display name for the current mode.
	pub fn mode_name(&self) -> &'static str {
		self.buffer().input.mode_name()
	}

	fn handle_window_focus_change(&mut self, old_focus: FocusTarget, new_focus: &FocusTarget) {
		let old_window = match old_focus {
			FocusTarget::Buffer { window, .. } => Some(window),
			FocusTarget::Panel(_) => None,
		};
		let new_window = match new_focus {
			FocusTarget::Buffer { window, .. } => Some(*window),
			FocusTarget::Panel(_) => None,
		};

		if old_window != new_window {
			if let Some(window) = old_window {
				emit_hook_sync_with(
					&HookContext::new(
						HookEventData::WindowFocusChanged {
							window_id: xeno_registry::WindowId(window.0),
							focused: false,
						},
						Some(&self.extensions),
					),
					&mut self.hook_runtime,
				);
			}
			if let Some(window) = new_window {
				emit_hook_sync_with(
					&HookContext::new(
						HookEventData::WindowFocusChanged {
							window_id: xeno_registry::WindowId(window.0),
							focused: true,
						},
						Some(&self.extensions),
					),
					&mut self.hook_runtime,
				);
			}
		}

		if let Some(window) = old_window
			&& old_window != new_window
		{
			let should_close = matches!(
				self.windows.get(window),
				Some(Window::Floating(floating)) if floating.dismiss_on_blur
			);
			if should_close {
				if Some(window) == self.palette.window_id() {
					self.close_palette();
				} else {
					self.close_floating_window(window);
				}
			}
		}
	}
}
