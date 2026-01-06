//! Hover popup for displaying LSP hover information.
//!
//! This module provides the [`HoverPopup`] type which displays type information
//! and documentation from LSP hover responses.

use xeno_lsp::lsp_types::{Hover, HoverContents, MarkedString, MarkupContent, MarkupKind};
use xeno_registry::themes::Theme;
use xeno_tui::Frame;
use xeno_tui::buffer::Buffer;
use xeno_tui::layout::Rect;
use xeno_tui::style::{Modifier, Style, Stylize};
use xeno_tui::symbols::border;
use xeno_tui::text::{Line, Span, Text};
use xeno_tui::widgets::Widget;

use super::{Popup, PopupAnchor, PopupEvent, PopupEventResult, SizeHints};

/// A popup for displaying LSP hover information.
///
/// HoverPopup displays type information and documentation from LSP hover responses.
/// It supports both plain text and markdown content, rendering them appropriately.
///
/// The popup is non-modal and dismisses on any key press or cursor movement.
pub struct HoverPopup {
	/// Unique identifier for this popup.
	id: String,
	/// Parsed content to display.
	content: HoverContent,
	/// Anchor position for the popup.
	anchor: PopupAnchor,
	/// Maximum width constraint.
	max_width: u16,
	/// Maximum height constraint.
	max_height: u16,
	/// Current scroll offset for tall content.
	scroll_offset: usize,
}

/// Parsed hover content ready for display.
#[derive(Debug, Clone)]
pub struct HoverContent {
	/// Lines of styled text to display.
	pub lines: Vec<HoverLine>,
}

/// A single line of hover content with optional styling.
#[derive(Debug, Clone)]
pub struct HoverLine {
	/// The text content.
	pub text: String,
	/// Whether this line is a code block.
	pub is_code: bool,
	/// Optional language for syntax highlighting.
	pub language: Option<String>,
}

impl HoverPopup {
	/// Creates a new hover popup from an LSP Hover response.
	pub fn from_hover(hover: Hover) -> Self {
		let content = parse_hover_contents(hover.contents);
		Self {
			id: "lsp-hover".to_string(),
			content,
			anchor: PopupAnchor::cursor_below(),
			max_width: 80,
			max_height: 20,
			scroll_offset: 0,
		}
	}

	/// Sets the anchor position for the popup.
	pub fn with_anchor(mut self, anchor: PopupAnchor) -> Self {
		self.anchor = anchor;
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

	/// Returns the number of content lines.
	pub fn line_count(&self) -> usize {
		self.content.lines.len()
	}

	/// Calculates the content dimensions.
	fn content_size(&self) -> (u16, u16) {
		let width = self
			.content
			.lines
			.iter()
			.map(|line| line.text.len() as u16)
			.max()
			.unwrap_or(0);

		let height = self.content.lines.len() as u16;

		// Add 2 for border
		(width + 2, height + 2)
	}

	/// Scrolls the content up by one line.
	fn scroll_up(&mut self) {
		self.scroll_offset = self.scroll_offset.saturating_sub(1);
	}

	/// Scrolls the content down by one line.
	fn scroll_down(&mut self, visible_height: usize) {
		let max_scroll = self.content.lines.len().saturating_sub(visible_height);
		if self.scroll_offset < max_scroll {
			self.scroll_offset += 1;
		}
	}

	/// Converts content to styled Text for rendering.
	fn to_styled_text(&self, theme: &Theme) -> Text<'static> {
		let mut lines = Vec::new();

		for hover_line in &self.content.lines {
			let style = if hover_line.is_code {
				// Code blocks get a slightly different background
				Style::default()
					.fg(theme.colors.popup.fg)
					.add_modifier(Modifier::BOLD)
			} else {
				Style::default().fg(theme.colors.popup.fg)
			};

			lines.push(Line::from(Span::styled(hover_line.text.clone(), style)));
		}

		Text::from(lines)
	}
}

impl Popup for HoverPopup {
	fn id(&self) -> &str {
		&self.id
	}

	fn anchor(&self) -> PopupAnchor {
		self.anchor
	}

	fn size_hints(&self) -> SizeHints {
		let (content_width, content_height) = self.content_size();
		SizeHints {
			min_width: 10,
			min_height: 3,
			max_width: self.max_width,
			max_height: self.max_height,
			preferred_width: content_width.min(self.max_width),
			preferred_height: content_height.min(self.max_height),
		}
	}

	fn handle_event(&mut self, event: PopupEvent) -> PopupEventResult {
		match event {
			PopupEvent::Key(key) => {
				use termina::event::KeyCode;
				match key.code {
					// Allow scrolling
					KeyCode::Up | KeyCode::Char('k') => {
						self.scroll_up();
						PopupEventResult::consumed()
					}
					KeyCode::Down | KeyCode::Char('j') => {
						// Use max_height - 2 (border) as visible height
						self.scroll_down((self.max_height.saturating_sub(2)) as usize);
						PopupEventResult::consumed()
					}
					// Any other key dismisses
					_ => PopupEventResult::dismissed(),
				}
			}
			// Cursor movement dismisses the popup
			PopupEvent::CursorMoved => PopupEventResult::dismissed(),
			// Mouse scroll events
			PopupEvent::Mouse(mouse) => {
				use termina::event::MouseEventKind;
				match mouse.kind {
					MouseEventKind::ScrollUp => {
						self.scroll_up();
						PopupEventResult::consumed()
					}
					MouseEventKind::ScrollDown => {
						self.scroll_down((self.max_height.saturating_sub(2)) as usize);
						PopupEventResult::consumed()
					}
					_ => PopupEventResult::consumed(),
				}
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

		// Draw title
		let title = " Hover ";
		let title_area = Rect::new(area.x + 2, area.y, area.width.saturating_sub(4), 1);
		if title_area.width > 0 {
			let title_line = Line::from(title).fg(theme.colors.popup.title);
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
			let visible_height = content_area.height as usize;

			// Render each visible line with scrolling
			for (i, hover_line) in self
				.content
				.lines
				.iter()
				.skip(self.scroll_offset)
				.take(visible_height)
				.enumerate()
			{
				let style = if hover_line.is_code {
					Style::default()
						.fg(theme.colors.popup.fg)
						.add_modifier(Modifier::BOLD)
				} else {
					Style::default().fg(theme.colors.popup.fg)
				};

				// Truncate line if needed
				let display_text: String = hover_line
					.text
					.chars()
					.take(content_area.width as usize)
					.collect();

				let line_area = Rect::new(
					content_area.x,
					content_area.y + i as u16,
					content_area.width,
					1,
				);

				Line::from(Span::styled(display_text, style)).render(line_area, &mut buffer);
			}

			// Show scroll indicator if content is taller than visible area
			if self.content.lines.len() > visible_height {
				let indicator = if self.scroll_offset > 0
					&& self.scroll_offset + visible_height < self.content.lines.len()
				{
					"..."
				} else if self.scroll_offset > 0 {
					"^"
				} else {
					"v"
				};

				let indicator_x = area.x + area.width - 2;
				if let Some(cell) = buffer.cell_mut((indicator_x, area.y + area.height - 1)) {
					cell.set_symbol(indicator).set_fg(theme.colors.popup.border);
				}
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

/// Parses LSP HoverContents into displayable content.
fn parse_hover_contents(contents: HoverContents) -> HoverContent {
	let mut lines = Vec::new();

	match contents {
		HoverContents::Scalar(marked_string) => {
			parse_marked_string(&mut lines, marked_string);
		}
		HoverContents::Array(marked_strings) => {
			for (i, marked_string) in marked_strings.into_iter().enumerate() {
				if i > 0 {
					// Add separator between sections
					lines.push(HoverLine {
						text: String::new(),
						is_code: false,
						language: None,
					});
				}
				parse_marked_string(&mut lines, marked_string);
			}
		}
		HoverContents::Markup(markup) => {
			parse_markup_content(&mut lines, markup);
		}
	}

	// Remove trailing empty lines
	while lines.last().map(|l| l.text.is_empty()).unwrap_or(false) {
		lines.pop();
	}

	HoverContent { lines }
}

/// Parses a MarkedString into hover lines.
fn parse_marked_string(lines: &mut Vec<HoverLine>, marked_string: MarkedString) {
	match marked_string {
		MarkedString::String(text) => {
			// Plain text - might contain markdown
			parse_markdown_text(lines, &text);
		}
		MarkedString::LanguageString(ls) => {
			// Code block with language
			for line in ls.value.lines() {
				lines.push(HoverLine {
					text: line.to_string(),
					is_code: true,
					language: Some(ls.language.clone()),
				});
			}
		}
	}
}

/// Parses MarkupContent into hover lines.
fn parse_markup_content(lines: &mut Vec<HoverLine>, markup: MarkupContent) {
	match markup.kind {
		MarkupKind::PlainText => {
			for line in markup.value.lines() {
				lines.push(HoverLine {
					text: line.to_string(),
					is_code: false,
					language: None,
				});
			}
		}
		MarkupKind::Markdown => {
			parse_markdown_text(lines, &markup.value);
		}
	}
}

/// Parses markdown text into hover lines.
///
/// This is a simplified markdown parser that handles:
/// - Code blocks (``` ... ```)
/// - Inline code (`...`)
/// - Regular text
fn parse_markdown_text(lines: &mut Vec<HoverLine>, text: &str) {
	let mut in_code_block = false;
	let mut code_language = None;

	for line in text.lines() {
		// Check for code block delimiters
		if line.starts_with("```") {
			if in_code_block {
				// End of code block
				in_code_block = false;
				code_language = None;
			} else {
				// Start of code block
				in_code_block = true;
				// Extract language if specified
				let lang = line.trim_start_matches('`').trim();
				code_language = if lang.is_empty() {
					None
				} else {
					Some(lang.to_string())
				};
			}
			continue;
		}

		if in_code_block {
			lines.push(HoverLine {
				text: line.to_string(),
				is_code: true,
				language: code_language.clone(),
			});
		} else {
			// Process inline code and other markdown
			let processed = process_inline_markdown(line);
			lines.push(HoverLine {
				text: processed,
				is_code: false,
				language: None,
			});
		}
	}
}

/// Processes inline markdown elements in a line.
///
/// Currently handles:
/// - Inline code (`...`) - backticks are removed
/// - Bold (**...**) - asterisks are removed
/// - Headers (# ...) - hash symbols are removed
fn process_inline_markdown(line: &str) -> String {
	let mut result = String::new();
	let mut chars = line.chars().peekable();

	// Handle headers
	if line.starts_with('#') {
		let trimmed = line.trim_start_matches('#').trim_start();
		return trimmed.to_string();
	}

	while let Some(c) = chars.next() {
		match c {
			'`' => {
				// Inline code - collect until closing backtick
				let mut code = String::new();
				while let Some(&next) = chars.peek() {
					if next == '`' {
						chars.next();
						break;
					}
					code.push(chars.next().unwrap());
				}
				result.push_str(&code);
			}
			'*' => {
				// Check for bold
				if chars.peek() == Some(&'*') {
					chars.next();
					// Collect until closing **
					let mut bold = String::new();
					while let Some(next) = chars.next() {
						if next == '*' && chars.peek() == Some(&'*') {
							chars.next();
							break;
						}
						bold.push(next);
					}
					result.push_str(&bold);
				} else {
					// Single * for italic - just collect until closing *
					let mut italic = String::new();
					while let Some(next) = chars.next() {
						if next == '*' {
							break;
						}
						italic.push(next);
					}
					result.push_str(&italic);
				}
			}
			_ => result.push(c),
		}
	}

	result
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
	use lsp_types::{LanguageString, MarkedString, MarkupContent, MarkupKind};

	use super::*;

	#[test]
	fn test_parse_plain_text() {
		let contents = HoverContents::Scalar(MarkedString::String("Hello, world!".into()));
		let content = parse_hover_contents(contents);
		assert_eq!(content.lines.len(), 1);
		assert_eq!(content.lines[0].text, "Hello, world!");
		assert!(!content.lines[0].is_code);
	}

	#[test]
	fn test_parse_code_block() {
		let contents = HoverContents::Scalar(MarkedString::LanguageString(LanguageString {
			language: "rust".into(),
			value: "fn main() {}".into(),
		}));
		let content = parse_hover_contents(contents);
		assert_eq!(content.lines.len(), 1);
		assert_eq!(content.lines[0].text, "fn main() {}");
		assert!(content.lines[0].is_code);
		assert_eq!(content.lines[0].language, Some("rust".into()));
	}

	#[test]
	fn test_parse_markdown_code_block() {
		let markdown = "Some text\n```rust\nfn foo() {}\n```\nMore text";
		let contents = HoverContents::Markup(MarkupContent {
			kind: MarkupKind::Markdown,
			value: markdown.into(),
		});
		let content = parse_hover_contents(contents);

		assert_eq!(content.lines.len(), 3);
		assert_eq!(content.lines[0].text, "Some text");
		assert!(!content.lines[0].is_code);

		assert_eq!(content.lines[1].text, "fn foo() {}");
		assert!(content.lines[1].is_code);
		assert_eq!(content.lines[1].language, Some("rust".into()));

		assert_eq!(content.lines[2].text, "More text");
		assert!(!content.lines[2].is_code);
	}

	#[test]
	fn test_parse_inline_code() {
		let markdown = "Use `foo()` to call it";
		let contents = HoverContents::Markup(MarkupContent {
			kind: MarkupKind::Markdown,
			value: markdown.into(),
		});
		let content = parse_hover_contents(contents);

		assert_eq!(content.lines.len(), 1);
		assert_eq!(content.lines[0].text, "Use foo() to call it");
	}

	#[test]
	fn test_parse_headers() {
		let markdown = "# Header\n## Subheader\nNormal text";
		let contents = HoverContents::Markup(MarkupContent {
			kind: MarkupKind::Markdown,
			value: markdown.into(),
		});
		let content = parse_hover_contents(contents);

		assert_eq!(content.lines.len(), 3);
		assert_eq!(content.lines[0].text, "Header");
		assert_eq!(content.lines[1].text, "Subheader");
		assert_eq!(content.lines[2].text, "Normal text");
	}

	#[test]
	fn test_hover_popup_creation() {
		let hover = Hover {
			contents: HoverContents::Scalar(MarkedString::String("test".into())),
			range: None,
		};
		let popup = HoverPopup::from_hover(hover);

		assert_eq!(popup.id(), "lsp-hover");
		assert!(!popup.is_modal());
		assert!(popup.dismiss_on_cursor_move());
	}

	#[test]
	fn test_hover_popup_size_hints() {
		let hover = Hover {
			contents: HoverContents::Scalar(MarkedString::String("Hello".into())),
			range: None,
		};
		let popup = HoverPopup::from_hover(hover);
		let hints = popup.size_hints();

		assert!(hints.min_width >= 10);
		assert!(hints.min_height >= 3);
		assert!(hints.max_width <= 80);
	}

	#[test]
	fn test_hover_popup_array_contents() {
		let contents = HoverContents::Array(vec![
			MarkedString::String("First section".into()),
			MarkedString::String("Second section".into()),
		]);
		let content = parse_hover_contents(contents);

		// Should have both sections with a separator
		assert!(content.lines.len() >= 2);
	}
}
