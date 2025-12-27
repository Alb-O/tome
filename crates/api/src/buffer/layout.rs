//! Layout management for buffer splits.
//!
//! The `Layout` enum represents how buffers are arranged in the editor window.
//! It supports recursive splitting for complex layouts.
//!
//! The layout system is view-agnostic: it can contain text buffers, terminals,
//! or any other content type via the `BufferView` enum.

use super::BufferId;

/// Path to a split in the layout tree.
///
/// Each element indicates which branch to take: `false` for first child,
/// `true` for second child. An empty path refers to the root split.
///
/// This provides a stable way to identify splits that doesn't change
/// when ratios are adjusted during resize operations.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SplitPath(pub Vec<bool>);

/// Direction of a split.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SplitDirection {
	/// Horizontal split (buffers side by side).
	Horizontal,
	/// Vertical split (buffers stacked).
	Vertical,
}

/// Unique identifier for a terminal buffer.
///
/// Terminal IDs are assigned sequentially starting from 1 when terminals
/// are created via [`Editor::split_horizontal_terminal`] or
/// [`Editor::split_vertical_terminal`].
///
/// [`Editor::split_horizontal_terminal`]: crate::Editor::split_horizontal_terminal
/// [`Editor::split_vertical_terminal`]: crate::Editor::split_vertical_terminal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalId(pub u64);

/// A view in the layout - either a text buffer or a terminal.
///
/// This enum enables the layout system to manage heterogeneous content types
/// in splits. The editor tracks the focused view via this type, allowing
/// seamless navigation between text editing and terminal sessions.
///
/// # Focus Handling
///
/// When a terminal is focused, text-editing operations are unavailable.
/// Use [`Editor::is_text_focused`] or [`Editor::is_terminal_focused`] to
/// check focus type before operations.
///
/// [`Editor::is_text_focused`]: crate::Editor::is_text_focused
/// [`Editor::is_terminal_focused`]: crate::Editor::is_terminal_focused
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BufferView {
	/// A text buffer for document editing.
	Text(BufferId),
	/// An embedded terminal emulator.
	Terminal(TerminalId),
}

impl BufferView {
	/// Returns the text buffer ID if this is a text view.
	pub fn as_text(&self) -> Option<BufferId> {
		match self {
			BufferView::Text(id) => Some(*id),
			BufferView::Terminal(_) => None,
		}
	}

	/// Returns the terminal ID if this is a terminal view.
	pub fn as_terminal(&self) -> Option<TerminalId> {
		match self {
			BufferView::Text(_) => None,
			BufferView::Terminal(id) => Some(*id),
		}
	}

	/// Returns true if this is a text buffer view.
	pub fn is_text(&self) -> bool {
		matches!(self, BufferView::Text(_))
	}

	/// Returns true if this is a terminal view.
	pub fn is_terminal(&self) -> bool {
		matches!(self, BufferView::Terminal(_))
	}
}

impl From<BufferId> for BufferView {
	fn from(id: BufferId) -> Self {
		BufferView::Text(id)
	}
}

impl From<TerminalId> for BufferView {
	fn from(id: TerminalId) -> Self {
		BufferView::Terminal(id)
	}
}

/// Layout tree for buffer arrangement.
///
/// Represents how views (text buffers and terminals) are arranged in splits.
/// The layout is a binary tree where leaves are single views and internal
/// nodes are splits.
///
/// # Structure
///
/// ```text
/// Layout::Split
/// ├── first: Layout::Single(BufferView::Text(1))
/// └── second: Layout::Split
///     ├── first: Layout::Single(BufferView::Text(2))
///     └── second: Layout::Single(BufferView::Terminal(1))
/// ```
///
/// # Creating Layouts
///
/// Use the constructor methods rather than building variants directly:
///
/// ```ignore
/// let layout = Layout::hsplit(
///     Layout::text(buffer_id),
///     Layout::terminal(terminal_id),
/// );
/// ```
#[derive(Debug, Clone)]
pub enum Layout {
	/// A single buffer view (text or terminal).
	Single(BufferView),
	/// A split containing two child layouts.
	Split {
		/// Direction of the split (horizontal or vertical).
		direction: SplitDirection,
		/// Ratio of space given to first child (0.0 to 1.0).
		ratio: f32,
		/// First child (left for horizontal, top for vertical).
		first: Box<Layout>,
		/// Second child (right for horizontal, bottom for vertical).
		second: Box<Layout>,
	},
}

impl Layout {
	/// Creates a new single-view layout from any view type.
	pub fn single(view: impl Into<BufferView>) -> Self {
		Layout::Single(view.into())
	}

	/// Creates a new single-view layout for a text buffer.
	pub fn text(buffer_id: BufferId) -> Self {
		Layout::Single(BufferView::Text(buffer_id))
	}

	/// Creates a new single-view layout for a terminal.
	pub fn terminal(terminal_id: TerminalId) -> Self {
		Layout::Single(BufferView::Terminal(terminal_id))
	}

	/// Creates a horizontal split (side by side).
	pub fn hsplit(first: Layout, second: Layout) -> Self {
		Layout::Split {
			direction: SplitDirection::Horizontal,
			ratio: 0.5,
			first: Box::new(first),
			second: Box::new(second),
		}
	}

	/// Creates a vertical split (stacked).
	pub fn vsplit(first: Layout, second: Layout) -> Self {
		Layout::Split {
			direction: SplitDirection::Vertical,
			ratio: 0.5,
			first: Box::new(first),
			second: Box::new(second),
		}
	}

	/// Returns the first view in the layout.
	///
	/// For splits, this returns the first view found (leftmost/topmost).
	pub fn first_view(&self) -> BufferView {
		match self {
			Layout::Single(view) => *view,
			Layout::Split { first, .. } => first.first_view(),
		}
	}

	/// Returns the first text buffer ID if one exists.
	///
	/// For splits, traverses leftmost/topmost first.
	pub fn first_buffer(&self) -> Option<BufferId> {
		match self {
			Layout::Single(BufferView::Text(id)) => Some(*id),
			Layout::Single(BufferView::Terminal(_)) => None,
			Layout::Split { first, second, .. } => {
				first.first_buffer().or_else(|| second.first_buffer())
			}
		}
	}

	/// Returns all views in this layout.
	pub fn views(&self) -> Vec<BufferView> {
		match self {
			Layout::Single(view) => vec![*view],
			Layout::Split { first, second, .. } => {
				let mut views = first.views();
				views.extend(second.views());
				views
			}
		}
	}

	/// Returns all text buffer IDs in this layout.
	pub fn buffer_ids(&self) -> Vec<BufferId> {
		self.views()
			.into_iter()
			.filter_map(|v| v.as_text())
			.collect()
	}

	/// Returns all terminal IDs in this layout.
	pub fn terminal_ids(&self) -> Vec<TerminalId> {
		self.views()
			.into_iter()
			.filter_map(|v| v.as_terminal())
			.collect()
	}

	/// Checks if this layout contains a specific view.
	pub fn contains_view(&self, view: BufferView) -> bool {
		match self {
			Layout::Single(v) => *v == view,
			Layout::Split { first, second, .. } => {
				first.contains_view(view) || second.contains_view(view)
			}
		}
	}

	/// Checks if this layout contains a specific text buffer.
	pub fn contains(&self, buffer_id: BufferId) -> bool {
		self.contains_view(BufferView::Text(buffer_id))
	}

	/// Checks if this layout contains a specific terminal.
	pub fn contains_terminal(&self, terminal_id: TerminalId) -> bool {
		self.contains_view(BufferView::Terminal(terminal_id))
	}

	/// Replaces a view with a new layout (for splitting).
	///
	/// Returns true if the replacement was made.
	pub fn replace_view(&mut self, target: BufferView, new_layout: Layout) -> bool {
		match self {
			Layout::Single(view) if *view == target => {
				*self = new_layout;
				true
			}
			Layout::Single(_) => false,
			Layout::Split { first, second, .. } => {
				first.replace_view(target, new_layout.clone())
					|| second.replace_view(target, new_layout)
			}
		}
	}

	/// Replaces a buffer ID with a new layout (for splitting).
	///
	/// Returns true if the replacement was made.
	pub fn replace(&mut self, target: BufferId, new_layout: Layout) -> bool {
		self.replace_view(BufferView::Text(target), new_layout)
	}

	/// Removes a view from the layout, collapsing splits as needed.
	///
	/// Returns the new layout if the view was found and removed,
	/// or None if removing would leave no views.
	pub fn remove_view(&self, target: BufferView) -> Option<Layout> {
		match self {
			Layout::Single(view) if *view == target => None,
			Layout::Single(_) => Some(self.clone()),
			Layout::Split {
				direction,
				ratio,
				first,
				second,
			} => {
				let first_removed = first.remove_view(target);
				let second_removed = second.remove_view(target);

				match (first_removed, second_removed) {
					(None, None) => None,
					(Some(layout), None) | (None, Some(layout)) => Some(layout),
					(Some(f), Some(s)) => Some(Layout::Split {
						direction: *direction,
						ratio: *ratio,
						first: Box::new(f),
						second: Box::new(s),
					}),
				}
			}
		}
	}

	/// Removes a buffer from the layout, collapsing splits as needed.
	///
	/// Returns the new layout if the buffer was found and removed,
	/// or None if removing would leave no buffers.
	pub fn remove(&self, target: BufferId) -> Option<Layout> {
		self.remove_view(BufferView::Text(target))
	}

	/// Removes a terminal from the layout, collapsing splits as needed.
	pub fn remove_terminal(&self, target: TerminalId) -> Option<Layout> {
		self.remove_view(BufferView::Terminal(target))
	}

	/// Counts the number of views in this layout.
	pub fn count(&self) -> usize {
		match self {
			Layout::Single(_) => 1,
			Layout::Split { first, second, .. } => first.count() + second.count(),
		}
	}

	/// Returns the next view in the layout order.
	///
	/// Used for `Ctrl+w w` navigation.
	pub fn next_view(&self, current: BufferView) -> BufferView {
		let views = self.views();
		if views.is_empty() {
			return current;
		}

		let current_idx = views.iter().position(|&v| v == current).unwrap_or(0);
		let next_idx = (current_idx + 1) % views.len();
		views[next_idx]
	}

	/// Returns the previous view in the layout order.
	pub fn prev_view(&self, current: BufferView) -> BufferView {
		let views = self.views();
		if views.is_empty() {
			return current;
		}

		let current_idx = views.iter().position(|&v| v == current).unwrap_or(0);
		let prev_idx = if current_idx == 0 {
			views.len() - 1
		} else {
			current_idx - 1
		};
		views[prev_idx]
	}

	/// Returns the next buffer ID in the layout order (text buffers only).
	///
	/// Used for `:bnext` navigation.
	pub fn next_buffer(&self, current: BufferId) -> BufferId {
		let ids = self.buffer_ids();
		if ids.is_empty() {
			return current;
		}

		let current_idx = ids.iter().position(|&id| id == current).unwrap_or(0);
		let next_idx = (current_idx + 1) % ids.len();
		ids[next_idx]
	}

	/// Returns the previous buffer ID in the layout order (text buffers only).
	///
	/// Used for `:bprev` navigation.
	pub fn prev_buffer(&self, current: BufferId) -> BufferId {
		let ids = self.buffer_ids();
		if ids.is_empty() {
			return current;
		}

		let current_idx = ids.iter().position(|&id| id == current).unwrap_or(0);
		let prev_idx = if current_idx == 0 {
			ids.len() - 1
		} else {
			current_idx - 1
		};
		ids[prev_idx]
	}

	/// Finds the view at the given screen coordinates.
	///
	/// Returns the view and its screen area if the coordinates fall within
	/// any view's bounds.
	pub fn view_at_position(
		&self,
		area: ratatui::layout::Rect,
		x: u16,
		y: u16,
	) -> Option<(BufferView, ratatui::layout::Rect)> {
		for (view, rect) in self.compute_view_areas(area) {
			if x >= rect.x
				&& x < rect.x + rect.width
				&& y >= rect.y
				&& y < rect.y + rect.height
			{
				return Some((view, rect));
			}
		}
		None
	}

	/// Computes the rectangular areas for each view in the layout.
	///
	/// Returns a vec of (BufferView, Rect) pairs representing the screen area
	/// assigned to each view.
	pub fn compute_view_areas(
		&self,
		area: ratatui::layout::Rect,
	) -> Vec<(BufferView, ratatui::layout::Rect)> {
		match self {
			Layout::Single(view) => vec![(*view, area)],
			Layout::Split {
				direction,
				ratio,
				first,
				second,
			} => {
				let (first_area, second_area) = Self::split_area(area, *direction, *ratio);
				let mut areas = first.compute_view_areas(first_area);
				areas.extend(second.compute_view_areas(second_area));
				areas
			}
		}
	}

	/// Computes the rectangular areas for each buffer in the layout.
	///
	/// Returns a vec of (BufferId, Rect) pairs representing the screen area
	/// assigned to each buffer.
	pub fn compute_areas(
		&self,
		area: ratatui::layout::Rect,
	) -> Vec<(BufferId, ratatui::layout::Rect)> {
		self.compute_view_areas(area)
			.into_iter()
			.filter_map(|(view, rect)| view.as_text().map(|id| (id, rect)))
			.collect()
	}

	/// Helper to split an area according to direction and ratio.
	fn split_area(
		area: ratatui::layout::Rect,
		direction: SplitDirection,
		ratio: f32,
	) -> (ratatui::layout::Rect, ratatui::layout::Rect) {
		match direction {
			SplitDirection::Horizontal => {
				let first_width = ((area.width as f32) * ratio).round() as u16;
				let second_width = area.width.saturating_sub(first_width).saturating_sub(1);
				let first_rect = ratatui::layout::Rect {
					x: area.x,
					y: area.y,
					width: first_width,
					height: area.height,
				};
				let second_rect = ratatui::layout::Rect {
					x: area.x + first_width + 1,
					y: area.y,
					width: second_width,
					height: area.height,
				};
				(first_rect, second_rect)
			}
			SplitDirection::Vertical => {
				let first_height = ((area.height as f32) * ratio).round() as u16;
				let second_height = area.height.saturating_sub(first_height).saturating_sub(1);
				let first_rect = ratatui::layout::Rect {
					x: area.x,
					y: area.y,
					width: area.width,
					height: first_height,
				};
				let second_rect = ratatui::layout::Rect {
					x: area.x,
					y: area.y + first_height + 1,
					width: area.width,
					height: second_height,
				};
				(first_rect, second_rect)
			}
		}
	}

	/// Finds the separator at the given screen coordinates.
	///
	/// Returns the separator's direction and rectangle if the coordinates
	/// fall within a separator's bounds.
	pub fn separator_at_position(
		&self,
		area: ratatui::layout::Rect,
		x: u16,
		y: u16,
	) -> Option<(SplitDirection, ratatui::layout::Rect)> {
		for (direction, _pos, rect) in self.separator_positions(area) {
			if x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height {
				return Some((direction, rect));
			}
		}
		None
	}

	/// Finds the separator and its path at the given screen coordinates.
	///
	/// Returns the separator's direction, rectangle, and the path to its split.
	/// The path is used to identify the split for resize operations.
	pub fn separator_with_path_at_position(
		&self,
		area: ratatui::layout::Rect,
		x: u16,
		y: u16,
	) -> Option<(SplitDirection, ratatui::layout::Rect, SplitPath)> {
		self.find_separator_with_path(area, x, y, SplitPath::default())
	}

	/// Recursive helper to find separator with its path.
	fn find_separator_with_path(
		&self,
		area: ratatui::layout::Rect,
		x: u16,
		y: u16,
		current_path: SplitPath,
	) -> Option<(SplitDirection, ratatui::layout::Rect, SplitPath)> {
		match self {
			Layout::Single(_) => None,
			Layout::Split {
				direction,
				ratio,
				first,
				second,
			} => {
				let (first_area, second_area, sep_rect) =
					Self::compute_split_areas(area, *direction, *ratio);

				// Check if point is on this separator
				if x >= sep_rect.x
					&& x < sep_rect.x + sep_rect.width
					&& y >= sep_rect.y
					&& y < sep_rect.y + sep_rect.height
				{
					return Some((*direction, sep_rect, current_path));
				}

				// Recurse into first child
				let mut first_path = current_path.clone();
				first_path.0.push(false);
				if let Some(result) = first.find_separator_with_path(first_area, x, y, first_path) {
					return Some(result);
				}

				// Recurse into second child
				let mut second_path = current_path;
				second_path.0.push(true);
				second.find_separator_with_path(second_area, x, y, second_path)
			}
		}
	}

	/// Resizes the split at the given path.
	///
	/// The new ratio is calculated based on the mouse position relative to
	/// the split's current area.
	///
	/// Returns true if a resize was performed.
	pub fn resize_at_path(
		&mut self,
		area: ratatui::layout::Rect,
		path: &SplitPath,
		mouse_x: u16,
		mouse_y: u16,
	) -> bool {
		self.do_resize_at_path(area, &path.0, mouse_x, mouse_y)
	}

	/// Recursive helper to find and resize the split by path.
	fn do_resize_at_path(
		&mut self,
		area: ratatui::layout::Rect,
		path: &[bool],
		mouse_x: u16,
		mouse_y: u16,
	) -> bool {
		match self {
			Layout::Single(_) => false,
			Layout::Split {
				direction,
				ratio,
				first,
				second,
			} => {
				// If path is empty, this is the target split
				if path.is_empty() {
					// Calculate new ratio based on mouse position
					let new_ratio = match direction {
						SplitDirection::Horizontal => {
							// Mouse x position relative to area start
							let relative_x = mouse_x.saturating_sub(area.x);
							// Clamp to valid range (leave room for separator)
							let clamped = relative_x.clamp(1, area.width.saturating_sub(2));
							clamped as f32 / area.width as f32
						}
						SplitDirection::Vertical => {
							// Mouse y position relative to area start
							let relative_y = mouse_y.saturating_sub(area.y);
							// Clamp to valid range
							let clamped = relative_y.clamp(1, area.height.saturating_sub(2));
							clamped as f32 / area.height as f32
						}
					};

					// Clamp ratio to reasonable bounds
					*ratio = new_ratio.clamp(0.1, 0.9);
					return true;
				}

				// Follow the path
				let (first_area, second_area, _) =
					Self::compute_split_areas(area, *direction, *ratio);
				let remaining_path = &path[1..];

				if path[0] {
					second.do_resize_at_path(second_area, remaining_path, mouse_x, mouse_y)
				} else {
					first.do_resize_at_path(first_area, remaining_path, mouse_x, mouse_y)
				}
			}
		}
	}

	/// Gets the current separator rect for a split at the given path.
	///
	/// Used to determine which separator to highlight during drag.
	pub fn separator_rect_at_path(
		&self,
		area: ratatui::layout::Rect,
		path: &SplitPath,
	) -> Option<(SplitDirection, ratatui::layout::Rect)> {
		self.do_get_separator_at_path(area, &path.0)
	}

	/// Recursive helper to get separator rect by path.
	fn do_get_separator_at_path(
		&self,
		area: ratatui::layout::Rect,
		path: &[bool],
	) -> Option<(SplitDirection, ratatui::layout::Rect)> {
		match self {
			Layout::Single(_) => None,
			Layout::Split {
				direction,
				ratio,
				first,
				second,
			} => {
				let (first_area, second_area, sep_rect) =
					Self::compute_split_areas(area, *direction, *ratio);

				// If path is empty, return this separator
				if path.is_empty() {
					return Some((*direction, sep_rect));
				}

				// Follow the path
				let remaining_path = &path[1..];
				if path[0] {
					second.do_get_separator_at_path(second_area, remaining_path)
				} else {
					first.do_get_separator_at_path(first_area, remaining_path)
				}
			}
		}
	}

	/// Helper to compute split areas (extracted for reuse).
	fn compute_split_areas(
		area: ratatui::layout::Rect,
		direction: SplitDirection,
		ratio: f32,
	) -> (
		ratatui::layout::Rect,
		ratatui::layout::Rect,
		ratatui::layout::Rect,
	) {
		match direction {
			SplitDirection::Horizontal => {
				let first_width = ((area.width as f32) * ratio).round() as u16;
				let first_rect = ratatui::layout::Rect {
					x: area.x,
					y: area.y,
					width: first_width,
					height: area.height,
				};
				let second_rect = ratatui::layout::Rect {
					x: area.x + first_width + 1,
					y: area.y,
					width: area.width.saturating_sub(first_width).saturating_sub(1),
					height: area.height,
				};
				let sep = ratatui::layout::Rect {
					x: area.x + first_width,
					y: area.y,
					width: 1,
					height: area.height,
				};
				(first_rect, second_rect, sep)
			}
			SplitDirection::Vertical => {
				let first_height = ((area.height as f32) * ratio).round() as u16;
				let first_rect = ratatui::layout::Rect {
					x: area.x,
					y: area.y,
					width: area.width,
					height: first_height,
				};
				let second_rect = ratatui::layout::Rect {
					x: area.x,
					y: area.y + first_height + 1,
					width: area.width,
					height: area.height.saturating_sub(first_height).saturating_sub(1),
				};
				let sep = ratatui::layout::Rect {
					x: area.x,
					y: area.y + first_height,
					width: area.width,
					height: 1,
				};
				(first_rect, second_rect, sep)
			}
		}
	}

	/// Returns the separator positions for rendering.
	///
	/// Each separator is represented as (direction, position) where position
	/// is the x coordinate for horizontal splits or y for vertical splits.
	pub fn separator_positions(
		&self,
		area: ratatui::layout::Rect,
	) -> Vec<(SplitDirection, u16, ratatui::layout::Rect)> {
		match self {
			Layout::Single(_) => vec![],
			Layout::Split {
				direction,
				ratio,
				first,
				second,
			} => {
				let (first_area, second_area, sep_rect) =
					Self::compute_split_areas(area, *direction, *ratio);

				let mut separators = vec![(*direction, sep_rect.x, sep_rect)];
				separators.extend(first.separator_positions(first_area));
				separators.extend(second.separator_positions(second_area));
				separators
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_single_layout() {
		let layout = Layout::single(BufferId(1));
		assert_eq!(layout.first_buffer(), Some(BufferId(1)));
		assert_eq!(layout.buffer_ids(), vec![BufferId(1)]);
		assert!(layout.contains(BufferId(1)));
		assert!(!layout.contains(BufferId(2)));
	}

	#[test]
	fn test_hsplit() {
		let layout = Layout::hsplit(Layout::single(BufferId(1)), Layout::single(BufferId(2)));

		assert_eq!(layout.first_buffer(), Some(BufferId(1)));
		assert_eq!(layout.buffer_ids(), vec![BufferId(1), BufferId(2)]);
		assert!(layout.contains(BufferId(1)));
		assert!(layout.contains(BufferId(2)));
		assert!(!layout.contains(BufferId(3)));
	}

	#[test]
	fn test_next_prev_buffer() {
		let layout = Layout::hsplit(Layout::single(BufferId(1)), Layout::single(BufferId(2)));

		assert_eq!(layout.next_buffer(BufferId(1)), BufferId(2));
		assert_eq!(layout.next_buffer(BufferId(2)), BufferId(1));
		assert_eq!(layout.prev_buffer(BufferId(1)), BufferId(2));
		assert_eq!(layout.prev_buffer(BufferId(2)), BufferId(1));
	}

	#[test]
	fn test_remove_buffer() {
		let layout = Layout::hsplit(Layout::single(BufferId(1)), Layout::single(BufferId(2)));

		let after_remove = layout.remove(BufferId(1)).unwrap();
		assert_eq!(after_remove.buffer_ids(), vec![BufferId(2)]);

		// Removing the only buffer returns None
		let single = Layout::single(BufferId(1));
		assert!(single.remove(BufferId(1)).is_none());
	}
}
