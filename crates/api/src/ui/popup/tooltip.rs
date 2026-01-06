//! Tooltip popup for displaying text content.

use xeno_registry::themes::Theme;
use xeno_tui::Frame;
use xeno_tui::buffer::Buffer;
use xeno_tui::layout::Rect;
use xeno_tui::style::Stylize;
use xeno_tui::symbols::border;
use xeno_tui::text::{Line, Text};
use xeno_tui::widgets::Widget;

use super::{Popup, PopupAnchor, PopupEvent, PopupEventResult, SizeHints};

/// A simple tooltip popup for displaying text content.
///
/// Tooltips are non-modal and dismiss on any key press or cursor movement.
/// They are typically used for hover information, quick documentation, etc.
pub struct TooltipPopup {
	/// Unique identifier for this popup.
	id: String,
	/// The text content to display.
	content: Text<'static>,
	/// Anchor position for the popup.
	anchor: PopupAnchor,
	/// Optional title for the popup.
	title: Option<String>,
	/// Maximum width constraint.
	max_width: u16,
	/// Maximum height constraint.
	max_height: u16,
}

impl TooltipPopup {
	/// Creates a new tooltip with the given ID and content.
	pub fn new(id: impl Into<String>, content: impl Into<Text<'static>>) -> Self {
		Self {
			id: id.into(),
			content: content.into(),
			anchor: PopupAnchor::cursor_below(),
			title: None,
			max_width: 60,
			max_height: 20,
		}
	}

	/// Sets the anchor position for the tooltip.
	pub fn with_anchor(mut self, anchor: PopupAnchor) -> Self {
		self.anchor = anchor;
		self
	}

	/// Sets an optional title for the tooltip.
	pub fn with_title(mut self, title: impl Into<String>) -> Self {
		self.title = Some(title.into());
		self
	}

	/// Sets the maximum width constraint.
	pub fn with_max_width(mut self, max_width: u16) -> Self {
		self.max_width = max_width;
		self
	}

	/// Sets the maximum height constraint.
	pub fn with_max_height(mut self, max_height: u16) -> Self {
		self.max_height = max_height;
		self
	}

	/// Calculates the content dimensions.
	fn content_size(&self) -> (u16, u16) {
		let width = self
			.content
			.lines
			.iter()
			.map(|line| line.width() as u16)
			.max()
			.unwrap_or(0);

		let height = self.content.lines.len() as u16;

		// Add 2 for border
		(width + 2, height + 2)
	}
}

impl Popup for TooltipPopup {
	fn id(&self) -> &str {
		&self.id
	}

	fn anchor(&self) -> PopupAnchor {
		self.anchor
	}

	fn size_hints(&self) -> SizeHints {
		let (content_width, content_height) = self.content_size();
		SizeHints {
			min_width: 5,
			min_height: 3,
			max_width: self.max_width,
			max_height: self.max_height,
			preferred_width: content_width.min(self.max_width),
			preferred_height: content_height.min(self.max_height),
		}
	}

	fn handle_event(&mut self, event: PopupEvent) -> PopupEventResult {
		match event {
			// Any key dismisses the tooltip
			PopupEvent::Key(_) => PopupEventResult::dismissed(),
			// Cursor movement dismisses the tooltip
			PopupEvent::CursorMoved => PopupEventResult::dismissed(),
			// Mouse events inside the tooltip are consumed but don't dismiss
			PopupEvent::Mouse(_) => PopupEventResult::consumed(),
			// Explicit dismiss request
			PopupEvent::Dismiss => PopupEventResult::dismissed(),
		}
	}

	fn render(&self, frame: &mut Frame, area: Rect, theme: &Theme) {
		if area.width < 3 || area.height < 3 {
			return;
		}

		// Clear the area first
		let mut buffer = Buffer::empty(area);
		for y in area.y..area.y + area.height {
			for x in area.x..area.x + area.width {
				if let Some(cell) = buffer.cell_mut((x, y)) {
					cell.set_symbol(" ")
						.set_bg(theme.colors.popup.bg)
						.set_fg(theme.colors.popup.fg);
				}
			}
		}

		// Draw border
		draw_border(&mut buffer, area, theme);

		// Draw title if present
		if let Some(title) = &self.title {
			let title_area = Rect::new(area.x + 2, area.y, area.width.saturating_sub(4), 1);
			if title_area.width > 0 {
				let title_text = format!(" {} ", title);
				let title_line = Line::from(title_text).fg(theme.colors.popup.title);
				title_line.render(title_area, &mut buffer);
			}
		}

		// Render content inside the border
		let content_area = Rect::new(
			area.x + 1,
			area.y + 1,
			area.width.saturating_sub(2),
			area.height.saturating_sub(2),
		);

		if content_area.width > 0 && content_area.height > 0 {
			// Render each line, truncating if necessary
			for (i, line) in self.content.lines.iter().enumerate() {
				if i as u16 >= content_area.height {
					break;
				}
				let line_area = Rect::new(
					content_area.x,
					content_area.y + i as u16,
					content_area.width,
					1,
				);
				line.clone().render(line_area, &mut buffer);
			}
		}

		// Merge buffer into frame
		frame.render_widget(BufferWidget(buffer), area);
	}

	fn is_modal(&self) -> bool {
		false
	}

	fn dismiss_on_cursor_move(&self) -> bool {
		true
	}

	fn as_any(&self) -> &dyn std::any::Any {
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
}

/// Helper to draw a border around an area.
fn draw_border(buffer: &mut Buffer, area: Rect, theme: &Theme) {
	let style = xeno_tui::style::Style::default()
		.fg(theme.colors.popup.border)
		.bg(theme.colors.popup.bg);

	let x = area.x;
	let y = area.y;
	let width = area.width;
	let height = area.height;

	// Corners
	if let Some(cell) = buffer.cell_mut((x, y)) {
		cell.set_symbol(border::ROUNDED.top_left).set_style(style);
	}
	if let Some(cell) = buffer.cell_mut((x + width - 1, y)) {
		cell.set_symbol(border::ROUNDED.top_right).set_style(style);
	}
	if let Some(cell) = buffer.cell_mut((x, y + height - 1)) {
		cell.set_symbol(border::ROUNDED.bottom_left)
			.set_style(style);
	}
	if let Some(cell) = buffer.cell_mut((x + width - 1, y + height - 1)) {
		cell.set_symbol(border::ROUNDED.bottom_right)
			.set_style(style);
	}

	// Horizontal edges
	for xi in (x + 1)..(x + width - 1) {
		if let Some(cell) = buffer.cell_mut((xi, y)) {
			cell.set_symbol(border::ROUNDED.horizontal_top)
				.set_style(style);
		}
		if let Some(cell) = buffer.cell_mut((xi, y + height - 1)) {
			cell.set_symbol(border::ROUNDED.horizontal_bottom)
				.set_style(style);
		}
	}

	// Vertical edges
	for yi in (y + 1)..(y + height - 1) {
		if let Some(cell) = buffer.cell_mut((x, yi)) {
			cell.set_symbol(border::ROUNDED.vertical_left)
				.set_style(style);
		}
		if let Some(cell) = buffer.cell_mut((x + width - 1, yi)) {
			cell.set_symbol(border::ROUNDED.vertical_right)
				.set_style(style);
		}
	}
}

/// Widget wrapper to render a buffer directly.
struct BufferWidget(Buffer);

impl Widget for BufferWidget {
	fn render(self, _area: Rect, buf: &mut Buffer) {
		// Merge source buffer into target
		for y in 0..self.0.area().height {
			for x in 0..self.0.area().width {
				let src_x = self.0.area().x + x;
				let src_y = self.0.area().y + y;
				if let Some(cell) = self.0.cell((src_x, src_y))
					&& let Some(target) = buf.cell_mut((src_x, src_y))
				{
					*target = cell.clone();
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_tooltip_creation() {
		let tooltip = TooltipPopup::new("test", "Hello, World!");
		assert_eq!(tooltip.id(), "test");
		assert!(!tooltip.is_modal());
		assert!(tooltip.dismiss_on_cursor_move());
	}

	#[test]
	fn test_tooltip_with_title() {
		let tooltip = TooltipPopup::new("test", "Content").with_title("Title");
		assert!(tooltip.title.is_some());
	}

	#[test]
	fn test_size_hints() {
		let tooltip = TooltipPopup::new("test", "Hello");
		let hints = tooltip.size_hints();
		assert!(hints.min_width > 0);
		assert!(hints.min_height > 0);
	}
}
