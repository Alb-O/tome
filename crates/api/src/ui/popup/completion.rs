//! Completion popup for displaying LSP completion suggestions.
//!
//! This module provides the [`CompletionPopup`] type which displays a scrollable
//! list of completion items with filtering, selection, and insertion capabilities.

use termina::event::{KeyCode, KeyEvent, Modifiers, MouseEventKind};
use xeno_lsp::lsp_types::{CompletionItem, CompletionItemKind, CompletionResponse};
use xeno_registry::themes::Theme;
use xeno_tui::Frame;
use xeno_tui::buffer::Buffer;
use xeno_tui::layout::Rect;
use xeno_tui::style::{Color, Modifier, Style, Stylize};
use xeno_tui::symbols::border;
use xeno_tui::text::{Line, Span};
use xeno_tui::widgets::Widget;

use super::{Popup, PopupAnchor, PopupEvent, PopupEventResult, SizeHints};

/// Maximum number of visible items in the completion list.
const MAX_VISIBLE_ITEMS: usize = 10;

/// Maximum width for the completion popup.
const MAX_WIDTH: u16 = 60;

/// Minimum width for the completion popup.
const MIN_WIDTH: u16 = 20;

/// A popup for displaying LSP completion suggestions.
///
/// CompletionPopup displays a scrollable, filterable list of completion items
/// from the language server. It supports keyboard navigation, filtering as
/// the user types, and selecting items for insertion.
///
/// The popup is modal (captures all input) and does not dismiss on cursor movement.
pub struct CompletionPopup {
	/// All completion items received from the LSP.
	items: Vec<CompletionItem>,
	/// Indices into `items` for items matching the current filter.
	filtered: Vec<usize>,
	/// Index into `filtered` of the currently selected item.
	selected: usize,
	/// Current filter text for incremental filtering.
	filter_text: String,
	/// Scroll offset for the visible window.
	scroll_offset: usize,
	/// Anchor position for the popup.
	anchor: PopupAnchor,
	/// Starting column position (for determining replacement range).
	trigger_column: usize,
}

/// Result of accepting a completion item.
#[derive(Debug, Clone)]
pub struct CompletionAcceptResult {
	/// The text to insert, replacing from trigger position to cursor.
	pub insert_text: String,
	/// Additional text edits to apply (e.g., auto-imports).
	pub additional_edits: Vec<xeno_lsp::lsp_types::TextEdit>,
}

impl CompletionPopup {
	/// Creates a new completion popup from an LSP CompletionResponse.
	///
	/// # Arguments
	///
	/// * `response` - The LSP completion response
	/// * `filter_text` - Initial filter text (text typed since trigger)
	/// * `trigger_column` - Column where completion was triggered
	pub fn from_response(
		response: CompletionResponse,
		filter_text: String,
		trigger_column: usize,
	) -> Self {
		let items = match response {
			CompletionResponse::Array(items) => items,
			CompletionResponse::List(list) => list.items,
		};

		let mut popup = Self {
			items,
			filtered: Vec::new(),
			selected: 0,
			filter_text,
			scroll_offset: 0,
			anchor: PopupAnchor::cursor_below(),
			trigger_column,
		};

		popup.apply_filter();
		popup
	}

	/// Creates a new completion popup from a list of items.
	pub fn new(items: Vec<CompletionItem>, filter_text: String, trigger_column: usize) -> Self {
		let mut popup = Self {
			items,
			filtered: Vec::new(),
			selected: 0,
			filter_text,
			scroll_offset: 0,
			anchor: PopupAnchor::cursor_below(),
			trigger_column,
		};

		popup.apply_filter();
		popup
	}

	/// Returns whether there are any filtered items to display.
	pub fn has_items(&self) -> bool {
		!self.filtered.is_empty()
	}

	/// Returns the number of filtered items.
	pub fn item_count(&self) -> usize {
		self.filtered.len()
	}

	/// Returns the trigger column where completion started.
	pub fn trigger_column(&self) -> usize {
		self.trigger_column
	}

	/// Updates the filter text and reapplies filtering.
	pub fn set_filter(&mut self, filter: String) {
		self.filter_text = filter;
		self.apply_filter();
	}

	/// Returns the current filter text.
	pub fn filter_text(&self) -> &str {
		&self.filter_text
	}

	/// Applies the current filter to the items.
	fn apply_filter(&mut self) {
		let filter_lower = self.filter_text.to_lowercase();

		self.filtered = self
			.items
			.iter()
			.enumerate()
			.filter(|(_, item)| {
				// Use filterText if available, otherwise label
				let text = item
					.filter_text
					.as_ref()
					.unwrap_or(&item.label)
					.to_lowercase();

				// Fuzzy matching: filter characters appear in order
				if filter_lower.is_empty() {
					return true;
				}

				let mut filter_chars = filter_lower.chars().peekable();
				for c in text.chars() {
					if filter_chars.peek() == Some(&c) {
						filter_chars.next();
					}
					if filter_chars.peek().is_none() {
						return true;
					}
				}
				false
			})
			.map(|(idx, _)| idx)
			.collect();

		// Sort by relevance: prefix matches first, then by label length
		self.filtered.sort_by(|&a, &b| {
			let item_a = &self.items[a];
			let item_b = &self.items[b];

			let label_a = item_a
				.filter_text
				.as_ref()
				.unwrap_or(&item_a.label)
				.to_lowercase();
			let label_b = item_b
				.filter_text
				.as_ref()
				.unwrap_or(&item_b.label)
				.to_lowercase();

			let prefix_a = label_a.starts_with(&filter_lower);
			let prefix_b = label_b.starts_with(&filter_lower);

			match (prefix_a, prefix_b) {
				(true, false) => std::cmp::Ordering::Less,
				(false, true) => std::cmp::Ordering::Greater,
				_ => label_a.len().cmp(&label_b.len()),
			}
		});

		// Reset selection if out of bounds
		if self.selected >= self.filtered.len() {
			self.selected = 0;
		}
		self.scroll_offset = 0;
	}

	/// Selects the next item in the list.
	pub fn select_next(&mut self) {
		if self.filtered.is_empty() {
			return;
		}
		self.selected = (self.selected + 1) % self.filtered.len();
		self.ensure_visible();
	}

	/// Selects the previous item in the list.
	pub fn select_prev(&mut self) {
		if self.filtered.is_empty() {
			return;
		}
		if self.selected == 0 {
			self.selected = self.filtered.len() - 1;
		} else {
			self.selected -= 1;
		}
		self.ensure_visible();
	}

	/// Ensures the selected item is visible by adjusting scroll offset.
	fn ensure_visible(&mut self) {
		if self.selected < self.scroll_offset {
			self.scroll_offset = self.selected;
		} else if self.selected >= self.scroll_offset + MAX_VISIBLE_ITEMS {
			self.scroll_offset = self.selected.saturating_sub(MAX_VISIBLE_ITEMS - 1);
		}
	}

	/// Returns the currently selected completion item, if any.
	pub fn selected_item(&self) -> Option<&CompletionItem> {
		self.filtered
			.get(self.selected)
			.and_then(|&idx| self.items.get(idx))
	}

	/// Accepts the currently selected item, returning the text to insert.
	///
	/// Returns `None` if no item is selected or the list is empty.
	pub fn accept_selected(&self) -> Option<CompletionAcceptResult> {
		let item = self.selected_item()?;

		// Prefer insertText, fall back to label
		let insert_text = item
			.insert_text
			.clone()
			.or_else(|| {
				item.text_edit.as_ref().map(|edit| match edit {
					xeno_lsp::lsp_types::CompletionTextEdit::Edit(e) => e.new_text.clone(),
					xeno_lsp::lsp_types::CompletionTextEdit::InsertAndReplace(e) => {
						e.new_text.clone()
					}
				})
			})
			.unwrap_or_else(|| item.label.clone());

		let additional_edits = item.additional_text_edits.clone().unwrap_or_default();

		Some(CompletionAcceptResult {
			insert_text,
			additional_edits,
		})
	}

	/// Calculates the preferred dimensions for the popup.
	fn content_size(&self) -> (u16, u16) {
		let visible_count = self.filtered.len().min(MAX_VISIBLE_ITEMS);

		// Calculate width based on longest item
		let max_label_width = self
			.filtered
			.iter()
			.filter_map(|&idx| self.items.get(idx))
			.map(|item| item.label.len() + 4) // +4 for icon and spacing
			.max()
			.unwrap_or(MIN_WIDTH as usize);

		let width = (max_label_width as u16 + 2).clamp(MIN_WIDTH, MAX_WIDTH);
		let height = (visible_count as u16 + 2).max(3); // +2 for border

		(width, height)
	}
}

impl Popup for CompletionPopup {
	fn id(&self) -> &str {
		"lsp-completion"
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
			PopupEvent::Mouse(mouse) => {
				match mouse.kind {
					MouseEventKind::ScrollUp => {
						self.select_prev();
						PopupEventResult::consumed()
					}
					MouseEventKind::ScrollDown => {
						self.select_next();
						PopupEventResult::consumed()
					}
					MouseEventKind::Down(_) => {
						// Could implement click-to-select here
						PopupEventResult::consumed()
					}
					_ => PopupEventResult::consumed(),
				}
			}
			PopupEvent::CursorMoved => {
				// Don't dismiss on cursor move - completion stays open while typing
				PopupEventResult::not_consumed()
			}
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

		// Render content inside the border
		let content_area = Rect::new(
			area.x + 1,
			area.y + 1,
			area.width.saturating_sub(2),
			area.height.saturating_sub(2),
		);

		if content_area.width > 0 && content_area.height > 0 {
			self.render_items(&mut buffer, content_area, theme);
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
		false
	}

	fn as_any(&self) -> &dyn std::any::Any {
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self
	}
}

impl CompletionPopup {
	/// Handles key events for the completion popup.
	fn handle_key(&mut self, key: KeyEvent) -> PopupEventResult {
		match (key.code, key.modifiers) {
			// Completion list navigation (Up/Down, Ctrl-N/Ctrl-P)
			(KeyCode::Down, _) | (KeyCode::Char('n'), Modifiers::CONTROL) => {
				self.select_next();
				PopupEventResult::consumed()
			}
			(KeyCode::Up, _) | (KeyCode::Char('p'), Modifiers::CONTROL) => {
				self.select_prev();
				PopupEventResult::consumed()
			}

			// Accept selection
			(KeyCode::Tab, _) | (KeyCode::Enter, _) => {
				// Signal acceptance by dismissing - caller checks selected_item()
				PopupEventResult {
					consumed: true,
					dismiss: true,
				}
			}

			// Dismiss without accepting
			(KeyCode::Escape, _) => PopupEventResult::dismissed(),

			// Cursor-moving keys dismiss completion without accepting
			// These keys would move the cursor outside the completion context
			(KeyCode::Left, _)
			| (KeyCode::Right, _)
			| (KeyCode::Home, _)
			| (KeyCode::End, _)
			| (KeyCode::PageUp, _)
			| (KeyCode::PageDown, _) => {
				// Dismiss but don't consume - let the key through to move cursor
				PopupEventResult {
					consumed: false,
					dismiss: true,
				}
			}

			// Let other keys through (for typing to filter)
			_ => PopupEventResult::not_consumed(),
		}
	}

	/// Renders the completion items.
	fn render_items(&self, buffer: &mut Buffer, area: Rect, theme: &Theme) {
		let visible_end = (self.scroll_offset + area.height as usize).min(self.filtered.len());

		for (i, &item_idx) in self
			.filtered
			.iter()
			.skip(self.scroll_offset)
			.take(area.height as usize)
			.enumerate()
		{
			let item = &self.items[item_idx];
			let is_selected = self.scroll_offset + i == self.selected;

			let line_area = Rect::new(area.x, area.y + i as u16, area.width, 1);

			self.render_item(buffer, line_area, item, is_selected, theme);
		}

		// Fill remaining lines with empty space if needed
		let rendered = visible_end - self.scroll_offset;
		for i in rendered..area.height as usize {
			let y = area.y + i as u16;
			for x in area.x..area.x + area.width {
				if let Some(cell) = buffer.cell_mut((x, y)) {
					cell.set_symbol(" ").set_bg(theme.colors.popup.bg);
				}
			}
		}
	}

	/// Renders a single completion item.
	fn render_item(
		&self,
		buffer: &mut Buffer,
		area: Rect,
		item: &CompletionItem,
		is_selected: bool,
		theme: &Theme,
	) {
		let bg = if is_selected {
			theme.colors.popup.selection
		} else {
			theme.colors.popup.bg
		};

		let fg = theme.colors.popup.fg;

		// Get icon and color for the item kind
		let (icon, icon_color) = completion_kind_icon(item.kind);

		// Build the line: icon + label + detail (if fits)
		let mut spans = vec![
			Span::styled(format!("{} ", icon), Style::default().fg(icon_color).bg(bg)),
			Span::styled(&item.label, Style::default().fg(fg).bg(bg)),
		];

		// Add detail if there's room
		if let Some(detail) = &item.detail {
			let label_len = item.label.len() + 2; // icon + space
			let remaining = (area.width as usize).saturating_sub(label_len + 3);
			if remaining > 5 {
				let detail_text: String = if detail.len() > remaining {
					format!(" {:.}...", &detail[..remaining - 3])
				} else {
					format!(" {}", detail)
				};
				spans.push(Span::styled(
					detail_text,
					Style::default()
						.fg(theme.colors.popup.border)
						.bg(bg)
						.add_modifier(Modifier::DIM),
				));
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
		if self.filtered.len() <= MAX_VISIBLE_ITEMS {
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
		if self.scroll_offset + MAX_VISIBLE_ITEMS < self.filtered.len() {
			if let Some(cell) = buffer.cell_mut((area.x + area.width - 2, area.y + area.height - 1))
			{
				cell.set_symbol("v").set_style(style);
			}
		}
	}
}

/// Returns the icon and color for a completion item kind.
fn completion_kind_icon(kind: Option<CompletionItemKind>) -> (&'static str, Color) {
	match kind {
		Some(CompletionItemKind::TEXT) => ("ab", Color::White),
		Some(CompletionItemKind::METHOD) => ("fn", Color::Cyan),
		Some(CompletionItemKind::FUNCTION) => ("fn", Color::Cyan),
		Some(CompletionItemKind::CONSTRUCTOR) => ("fn", Color::Cyan),
		Some(CompletionItemKind::FIELD) => ("fd", Color::Yellow),
		Some(CompletionItemKind::VARIABLE) => ("va", Color::Blue),
		Some(CompletionItemKind::CLASS) => ("cl", Color::Green),
		Some(CompletionItemKind::INTERFACE) => ("if", Color::Green),
		Some(CompletionItemKind::MODULE) => ("md", Color::Magenta),
		Some(CompletionItemKind::PROPERTY) => ("pr", Color::Yellow),
		Some(CompletionItemKind::UNIT) => ("un", Color::White),
		Some(CompletionItemKind::VALUE) => ("vl", Color::White),
		Some(CompletionItemKind::ENUM) => ("en", Color::Green),
		Some(CompletionItemKind::KEYWORD) => ("kw", Color::Red),
		Some(CompletionItemKind::SNIPPET) => ("sn", Color::LightMagenta),
		Some(CompletionItemKind::COLOR) => ("co", Color::White),
		Some(CompletionItemKind::FILE) => ("fi", Color::White),
		Some(CompletionItemKind::REFERENCE) => ("rf", Color::White),
		Some(CompletionItemKind::FOLDER) => ("fo", Color::White),
		Some(CompletionItemKind::ENUM_MEMBER) => ("em", Color::Green),
		Some(CompletionItemKind::CONSTANT) => ("ct", Color::Yellow),
		Some(CompletionItemKind::STRUCT) => ("st", Color::Green),
		Some(CompletionItemKind::EVENT) => ("ev", Color::White),
		Some(CompletionItemKind::OPERATOR) => ("op", Color::White),
		Some(CompletionItemKind::TYPE_PARAMETER) => ("tp", Color::Green),
		None | Some(_) => ("  ", Color::White),
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
	use super::*;

	fn make_item(label: &str, kind: Option<CompletionItemKind>) -> CompletionItem {
		CompletionItem {
			label: label.to_string(),
			kind,
			..Default::default()
		}
	}

	#[test]
	fn test_completion_popup_creation() {
		let items = vec![
			make_item("foo", Some(CompletionItemKind::FUNCTION)),
			make_item("bar", Some(CompletionItemKind::VARIABLE)),
			make_item("baz", Some(CompletionItemKind::STRUCT)),
		];

		let popup = CompletionPopup::new(items, String::new(), 0);
		assert_eq!(popup.id(), "lsp-completion");
		assert!(popup.has_items());
		assert_eq!(popup.item_count(), 3);
	}

	#[test]
	fn test_filtering() {
		let items = vec![
			make_item("foo", Some(CompletionItemKind::FUNCTION)),
			make_item("bar", Some(CompletionItemKind::VARIABLE)),
			make_item("foobar", Some(CompletionItemKind::STRUCT)),
		];

		let mut popup = CompletionPopup::new(items, String::new(), 0);
		assert_eq!(popup.item_count(), 3);

		popup.set_filter("fo".to_string());
		assert_eq!(popup.item_count(), 2);

		popup.set_filter("foob".to_string());
		assert_eq!(popup.item_count(), 1);
		assert_eq!(popup.selected_item().unwrap().label, "foobar");
	}

	#[test]
	fn test_navigation() {
		let items = vec![
			make_item("aaa", None),
			make_item("bbb", None),
			make_item("ccc", None),
		];

		let mut popup = CompletionPopup::new(items, String::new(), 0);
		assert_eq!(popup.selected_item().unwrap().label, "aaa");

		popup.select_next();
		assert_eq!(popup.selected_item().unwrap().label, "bbb");

		popup.select_next();
		assert_eq!(popup.selected_item().unwrap().label, "ccc");

		popup.select_next(); // Wraps around
		assert_eq!(popup.selected_item().unwrap().label, "aaa");

		popup.select_prev(); // Wraps around backward
		assert_eq!(popup.selected_item().unwrap().label, "ccc");
	}

	#[test]
	fn test_accept_selected() {
		let mut item = make_item("my_function", Some(CompletionItemKind::FUNCTION));
		item.insert_text = Some("my_function()".to_string());

		let popup = CompletionPopup::new(vec![item], String::new(), 0);
		let result = popup.accept_selected().unwrap();

		assert_eq!(result.insert_text, "my_function()");
	}

	#[test]
	fn test_empty_filter_shows_all() {
		let items = vec![
			make_item("alpha", None),
			make_item("beta", None),
			make_item("gamma", None),
		];

		let popup = CompletionPopup::new(items, String::new(), 0);
		assert_eq!(popup.item_count(), 3);
	}

	#[test]
	fn test_no_matches() {
		let items = vec![make_item("foo", None), make_item("bar", None)];

		let mut popup = CompletionPopup::new(items, String::new(), 0);
		popup.set_filter("xyz".to_string());
		assert!(!popup.has_items());
		assert!(popup.selected_item().is_none());
	}
}
