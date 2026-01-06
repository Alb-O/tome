//! Location picker popup for displaying multiple LSP location results.
//!
//! This module provides the [`LocationPickerPopup`] type which displays a list
//! of locations (e.g., multiple definitions, implementations) and allows the
//! user to select one to navigate to.

use std::path::PathBuf;

use termina::event::{KeyCode, KeyEvent, MouseEventKind};
use xeno_lsp::lsp_types::{GotoDefinitionResponse, Location, LocationLink, Url};
use xeno_registry::themes::Theme;
use xeno_tui::Frame;
use xeno_tui::buffer::Buffer;
use xeno_tui::layout::Rect;
use xeno_tui::style::{Modifier, Style, Stylize};
use xeno_tui::symbols::border;
use xeno_tui::text::{Line, Span};
use xeno_tui::widgets::Widget;

use super::{Popup, PopupAnchor, PopupEvent, PopupEventResult, SizeHints};

/// Maximum number of visible items in the location list.
const MAX_VISIBLE_ITEMS: usize = 10;

/// Maximum width for the location picker popup.
const MAX_WIDTH: u16 = 80;

/// Minimum width for the location picker popup.
const MIN_WIDTH: u16 = 40;

/// A location entry in the picker.
#[derive(Debug, Clone)]
pub struct LocationEntry {
	/// The URI of the location.
	pub uri: Url,
	/// The file path (extracted from URI).
	pub path: Option<PathBuf>,
	/// The 0-indexed line number.
	pub line: u32,
	/// The 0-indexed column number.
	pub col: u32,
	/// Optional context/preview text.
	pub context: Option<String>,
}

impl LocationEntry {
	/// Creates a location entry from an LSP location.
	pub fn from_location(location: &Location) -> Self {
		let path = location.uri.to_file_path().ok();
		Self {
			uri: location.uri.clone(),
			path,
			line: location.range.start.line,
			col: location.range.start.character,
			context: None,
		}
	}

	/// Creates a location entry from an LSP location link.
	pub fn from_location_link(link: &LocationLink) -> Self {
		let path = link.target_uri.to_file_path().ok();
		Self {
			uri: link.target_uri.clone(),
			path,
			line: link.target_selection_range.start.line,
			col: link.target_selection_range.start.character,
			context: None,
		}
	}

	/// Returns a display label for the location.
	pub fn display_label(&self) -> String {
		let filename = self
			.path
			.as_ref()
			.and_then(|p| p.file_name())
			.and_then(|n| n.to_str())
			.unwrap_or_else(|| self.uri.as_str());

		format!("{}:{}:{}", filename, self.line + 1, self.col + 1)
	}

	/// Converts to an LSP Location.
	pub fn to_location(&self) -> Location {
		Location {
			uri: self.uri.clone(),
			range: xeno_lsp::lsp_types::Range {
				start: xeno_lsp::lsp_types::Position {
					line: self.line,
					character: self.col,
				},
				end: xeno_lsp::lsp_types::Position {
					line: self.line,
					character: self.col,
				},
			},
		}
	}
}

/// A popup for selecting from multiple locations.
///
/// Used when goto definition returns multiple possible locations.
pub struct LocationPickerPopup {
	/// All location entries.
	entries: Vec<LocationEntry>,
	/// Index of the currently selected entry.
	selected: usize,
	/// Scroll offset for the visible window.
	scroll_offset: usize,
	/// Title for the popup.
	title: String,
	/// Anchor position for the popup.
	anchor: PopupAnchor,
}

impl LocationPickerPopup {
	/// Creates a location picker from a goto definition response.
	///
	/// Returns `None` if the response is empty.
	pub fn from_definition_response(response: GotoDefinitionResponse) -> Option<Self> {
		let entries: Vec<LocationEntry> = match response {
			GotoDefinitionResponse::Scalar(loc) => vec![LocationEntry::from_location(&loc)],
			GotoDefinitionResponse::Array(locs) => {
				locs.iter().map(LocationEntry::from_location).collect()
			}
			GotoDefinitionResponse::Link(links) => links
				.iter()
				.map(LocationEntry::from_location_link)
				.collect(),
		};

		if entries.is_empty() {
			return None;
		}

		// If only one entry, we don't need a picker
		if entries.len() == 1 {
			return None;
		}

		Some(Self {
			entries,
			selected: 0,
			scroll_offset: 0,
			title: "Definitions".to_string(),
			anchor: PopupAnchor::cursor_below(),
		})
	}

	/// Creates a location picker from a list of locations.
	///
	/// Returns `None` if the list is empty.
	pub fn from_locations(locations: Vec<Location>, title: &str) -> Option<Self> {
		if locations.is_empty() {
			return None;
		}

		// If only one location, we don't need a picker
		if locations.len() == 1 {
			return None;
		}

		let entries = locations.iter().map(LocationEntry::from_location).collect();

		Some(Self {
			entries,
			selected: 0,
			scroll_offset: 0,
			title: title.to_string(),
			anchor: PopupAnchor::cursor_below(),
		})
	}

	/// Returns whether there are entries to display.
	pub fn has_entries(&self) -> bool {
		!self.entries.is_empty()
	}

	/// Returns the number of entries.
	pub fn entry_count(&self) -> usize {
		self.entries.len()
	}

	/// Selects the next entry in the list.
	pub fn select_next(&mut self) {
		if self.entries.is_empty() {
			return;
		}
		self.selected = (self.selected + 1) % self.entries.len();
		self.ensure_visible();
	}

	/// Selects the previous entry in the list.
	pub fn select_prev(&mut self) {
		if self.entries.is_empty() {
			return;
		}
		if self.selected == 0 {
			self.selected = self.entries.len() - 1;
		} else {
			self.selected -= 1;
		}
		self.ensure_visible();
	}

	/// Ensures the selected entry is visible by adjusting scroll offset.
	fn ensure_visible(&mut self) {
		if self.selected < self.scroll_offset {
			self.scroll_offset = self.selected;
		} else if self.selected >= self.scroll_offset + MAX_VISIBLE_ITEMS {
			self.scroll_offset = self.selected.saturating_sub(MAX_VISIBLE_ITEMS - 1);
		}
	}

	/// Returns the currently selected entry, if any.
	pub fn selected_entry(&self) -> Option<&LocationEntry> {
		self.entries.get(self.selected)
	}

	/// Accepts the currently selected entry, returning its location.
	pub fn accept_selected(&self) -> Option<Location> {
		self.selected_entry().map(|e| e.to_location())
	}

	/// Calculates the preferred dimensions for the popup.
	fn content_size(&self) -> (u16, u16) {
		let visible_count = self.entries.len().min(MAX_VISIBLE_ITEMS);

		// Calculate width based on longest label
		let max_label_width = self
			.entries
			.iter()
			.map(|e| e.display_label().len() + 3) // +3 for icon and spacing
			.max()
			.unwrap_or(MIN_WIDTH as usize);

		let width = (max_label_width as u16 + 2).clamp(MIN_WIDTH, MAX_WIDTH);
		let height = (visible_count as u16 + 2).max(3); // +2 for border

		(width, height)
	}
}

impl Popup for LocationPickerPopup {
	fn id(&self) -> &str {
		"lsp-location-picker"
	}

	fn anchor(&self) -> PopupAnchor {
		self.anchor
	}

	fn size_hints(&self) -> SizeHints {
		let (width, height) = self.content_size();
		SizeHints {
			min_width: MIN_WIDTH,
			min_height: 3,
			max_width: MAX_WIDTH,
			max_height: (MAX_VISIBLE_ITEMS + 2) as u16,
			preferred_width: width,
			preferred_height: height,
		}
	}

	fn handle_event(&mut self, event: PopupEvent) -> PopupEventResult {
		match event {
			PopupEvent::Key(key) => self.handle_key(key),
			PopupEvent::Mouse(mouse) => match mouse.kind {
				MouseEventKind::ScrollUp => {
					self.select_prev();
					PopupEventResult::consumed()
				}
				MouseEventKind::ScrollDown => {
					self.select_next();
					PopupEventResult::consumed()
				}
				_ => PopupEventResult::consumed(),
			},
			PopupEvent::CursorMoved => PopupEventResult::not_consumed(),
			PopupEvent::Dismiss => PopupEventResult::dismissed(),
		}
	}

	fn render(&self, frame: &mut Frame, area: Rect, theme: &Theme) {
		if area.width < 3 || area.height < 3 {
			return;
		}

		// Clear the area with background color
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

		// Draw title
		let title = format!(" {} ({}) ", self.title, self.entries.len());
		let title_area = Rect::new(area.x + 2, area.y, area.width.saturating_sub(4), 1);
		if title_area.width > 0 {
			let title_line = Line::from(title.as_str()).fg(theme.colors.popup.title);
			title_line.render(title_area, &mut buffer);
		}

		// Render content inside the border
		let content_area = Rect::new(
			area.x + 1,
			area.y + 1,
			area.width.saturating_sub(2),
			area.height.saturating_sub(2),
		);

		if content_area.width > 0 && content_area.height > 0 {
			self.render_entries(&mut buffer, content_area, theme);
		}

		// Render scroll indicators if needed
		self.render_scroll_indicators(&mut buffer, area, theme);

		// Merge buffer into frame
		frame.render_widget(BufferWidget(buffer), area);
	}

	fn is_modal(&self) -> bool {
		true
	}

	fn dismiss_on_cursor_move(&self) -> bool {
		false // Don't dismiss on cursor move for picker
	}

	fn as_any(&self) -> &dyn std::any::Any {
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
}

impl LocationPickerPopup {
	/// Handles key events for the location picker popup.
	fn handle_key(&mut self, key: KeyEvent) -> PopupEventResult {
		match key.code {
			// Navigation
			KeyCode::Down | KeyCode::Char('j') => {
				self.select_next();
				PopupEventResult::consumed()
			}
			KeyCode::Up | KeyCode::Char('k') => {
				self.select_prev();
				PopupEventResult::consumed()
			}

			// Accept selection
			KeyCode::Enter => PopupEventResult {
				consumed: true,
				dismiss: true,
			},

			// Dismiss without accepting
			KeyCode::Escape | KeyCode::Char('q') => PopupEventResult::dismissed(),

			// Let other keys pass through
			_ => PopupEventResult::not_consumed(),
		}
	}

	/// Renders the location entries.
	fn render_entries(&self, buffer: &mut Buffer, area: Rect, theme: &Theme) {
		for (i, entry) in self
			.entries
			.iter()
			.skip(self.scroll_offset)
			.take(area.height as usize)
			.enumerate()
		{
			let is_selected = self.scroll_offset + i == self.selected;
			let line_area = Rect::new(area.x, area.y + i as u16, area.width, 1);
			self.render_entry(buffer, line_area, entry, is_selected, theme);
		}

		// Fill remaining lines with empty space if needed
		let rendered = self
			.entries
			.len()
			.saturating_sub(self.scroll_offset)
			.min(area.height as usize);
		for i in rendered..area.height as usize {
			let y = area.y + i as u16;
			for x in area.x..area.x + area.width {
				if let Some(cell) = buffer.cell_mut((x, y)) {
					cell.set_symbol(" ").set_bg(theme.colors.popup.bg);
				}
			}
		}
	}

	/// Renders a single location entry.
	fn render_entry(
		&self,
		buffer: &mut Buffer,
		area: Rect,
		entry: &LocationEntry,
		is_selected: bool,
		theme: &Theme,
	) {
		let bg = if is_selected {
			theme.colors.popup.selection
		} else {
			theme.colors.popup.bg
		};

		let fg = theme.colors.popup.fg;

		// Build the line: icon + location label
		let icon_span = Span::styled(
			" ",
			Style::default().fg(theme.colors.status.accent_fg).bg(bg),
		);
		let label = entry.display_label();
		let label_span = Span::styled(&label, Style::default().fg(fg).bg(bg));

		// Add full path hint if there's room
		let mut spans = vec![icon_span, label_span];

		if let Some(path) = &entry.path {
			let label_len = label.len() + 3;
			let remaining = (area.width as usize).saturating_sub(label_len + 3);
			if remaining > 10 {
				if let Some(parent) = path.parent() {
					let parent_str = parent.to_string_lossy();
					let truncated = if parent_str.len() > remaining - 3 {
						format!("...{}", &parent_str[parent_str.len() - (remaining - 6)..])
					} else {
						parent_str.to_string()
					};
					spans.push(Span::styled(
						format!(" {}", truncated),
						Style::default()
							.fg(theme.colors.status.dim_fg)
							.bg(bg)
							.add_modifier(Modifier::DIM),
					));
				}
			}
		}

		let line = Line::from(spans);

		// Clear line first
		for x in area.x..area.x + area.width {
			if let Some(cell) = buffer.cell_mut((x, area.y)) {
				cell.set_symbol(" ").set_bg(bg).set_fg(fg);
			}
		}

		// Render the line
		line.render(area, buffer);
	}

	/// Renders scroll indicators if content exceeds visible area.
	fn render_scroll_indicators(&self, buffer: &mut Buffer, area: Rect, theme: &Theme) {
		if self.entries.len() <= MAX_VISIBLE_ITEMS {
			return;
		}

		let style = Style::default()
			.fg(theme.colors.popup.border)
			.bg(theme.colors.popup.bg);

		// Up arrow if scrolled down
		if self.scroll_offset > 0 {
			if let Some(cell) = buffer.cell_mut((area.x + area.width - 2, area.y)) {
				cell.set_symbol("^").set_style(style);
			}
		}

		// Down arrow if more items below
		if self.scroll_offset + MAX_VISIBLE_ITEMS < self.entries.len() {
			if let Some(cell) = buffer.cell_mut((area.x + area.width - 2, area.y + area.height - 1))
			{
				cell.set_symbol("v").set_style(style);
			}
		}
	}
}

/// Helper to draw a border around an area.
fn draw_border(buffer: &mut Buffer, area: Rect, theme: &Theme) {
	let style = Style::default()
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
	use xeno_lsp::lsp_types::{Position, Range};

	use super::*;

	fn make_location(path: &str, line: u32) -> Location {
		Location {
			uri: Url::from_file_path(path).unwrap(),
			range: Range {
				start: Position { line, character: 0 },
				end: Position { line, character: 0 },
			},
		}
	}

	#[test]
	fn test_location_picker_creation() {
		let locations = vec![
			make_location("/tmp/test/foo.rs", 10),
			make_location("/tmp/test/bar.rs", 20),
		];

		let popup = LocationPickerPopup::from_locations(locations, "Definitions").unwrap();
		assert_eq!(popup.id(), "lsp-location-picker");
		assert!(popup.has_entries());
		assert_eq!(popup.entry_count(), 2);
	}

	#[test]
	fn test_location_picker_single_returns_none() {
		let locations = vec![make_location("/tmp/test/foo.rs", 10)];

		// Should return None for single location
		let popup = LocationPickerPopup::from_locations(locations, "Definitions");
		assert!(popup.is_none());
	}

	#[test]
	fn test_location_picker_empty_returns_none() {
		let popup = LocationPickerPopup::from_locations(vec![], "Definitions");
		assert!(popup.is_none());
	}

	#[test]
	fn test_navigation() {
		let locations = vec![
			make_location("/tmp/test/foo.rs", 10),
			make_location("/tmp/test/bar.rs", 20),
			make_location("/tmp/test/baz.rs", 30),
		];

		let mut popup = LocationPickerPopup::from_locations(locations, "Definitions").unwrap();
		assert_eq!(popup.selected_entry().unwrap().line, 10);

		popup.select_next();
		assert_eq!(popup.selected_entry().unwrap().line, 20);

		popup.select_next();
		assert_eq!(popup.selected_entry().unwrap().line, 30);

		popup.select_next(); // Wraps around
		assert_eq!(popup.selected_entry().unwrap().line, 10);

		popup.select_prev(); // Wraps around backward
		assert_eq!(popup.selected_entry().unwrap().line, 30);
	}

	#[test]
	fn test_accept_selected() {
		let locations = vec![
			make_location("/tmp/test/foo.rs", 10),
			make_location("/tmp/test/bar.rs", 20),
		];

		let popup = LocationPickerPopup::from_locations(locations, "Definitions").unwrap();
		let loc = popup.accept_selected().unwrap();
		assert_eq!(loc.range.start.line, 10);
	}
}
