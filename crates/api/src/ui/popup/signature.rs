//! Signature help popup for displaying function parameter hints.
//!
//! This module provides the [`SignaturePopup`] type which displays function
//! signatures with active parameter highlighting during function calls.

use termina::event::{KeyCode, KeyEvent, Modifiers, MouseEventKind};
use xeno_lsp::lsp_types::{
	Documentation, MarkupContent, MarkupKind, ParameterInformation, ParameterLabel, SignatureHelp,
	SignatureInformation,
};
use xeno_registry::themes::Theme;
use xeno_tui::Frame;
use xeno_tui::buffer::Buffer;
use xeno_tui::layout::Rect;
use xeno_tui::style::{Color, Modifier, Style, Stylize};
use xeno_tui::symbols::border;
use xeno_tui::text::{Line, Span};
use xeno_tui::widgets::Widget;

use super::{Popup, PopupAnchor, PopupEvent, PopupEventResult, SizeHints};

/// Maximum width for the signature popup.
const MAX_WIDTH: u16 = 80;

/// Maximum height for the signature popup.
const MAX_HEIGHT: u16 = 10;

/// A popup for displaying LSP signature help information.
///
/// SignaturePopup displays function signatures with the active parameter highlighted.
/// It supports multiple overloads which can be cycled through.
///
/// The popup stays open while typing in insert mode and doesn't dismiss on cursor movement.
pub struct SignaturePopup {
	/// All signatures from the LSP response.
	signatures: Vec<SignatureInformation>,
	/// Index of the currently active signature.
	active_signature: usize,
	/// Index of the currently active parameter (from LSP or derived).
	active_parameter: Option<usize>,
	/// Anchor position for the popup.
	anchor: PopupAnchor,
}

impl SignaturePopup {
	/// Creates a new signature popup from an LSP SignatureHelp response.
	pub fn from_signature_help(help: SignatureHelp) -> Option<Self> {
		if help.signatures.is_empty() {
			return None;
		}

		let active_signature = help.active_signature.unwrap_or(0) as usize;
		let active_parameter = help.active_parameter.map(|p| p as usize);
		let sig_count = help.signatures.len();

		Some(Self {
			signatures: help.signatures,
			active_signature: active_signature.min(sig_count.saturating_sub(1)),
			active_parameter,
			anchor: PopupAnchor::cursor_above(),
		})
	}

	/// Creates a new signature popup with explicit signatures.
	pub fn new(signatures: Vec<SignatureInformation>) -> Option<Self> {
		if signatures.is_empty() {
			return None;
		}

		Some(Self {
			signatures,
			active_signature: 0,
			active_parameter: None,
			anchor: PopupAnchor::cursor_above(),
		})
	}

	/// Sets the active parameter index.
	pub fn set_active_parameter(&mut self, index: Option<usize>) {
		self.active_parameter = index;
	}

	/// Returns the number of signatures.
	pub fn signature_count(&self) -> usize {
		self.signatures.len()
	}

	/// Returns the active signature index.
	pub fn active_signature_index(&self) -> usize {
		self.active_signature
	}

	/// Returns a reference to the active signature.
	pub fn active_signature(&self) -> Option<&SignatureInformation> {
		self.signatures.get(self.active_signature)
	}

	/// Cycles to the next signature (for overloads).
	pub fn next_signature(&mut self) {
		if self.signatures.len() > 1 {
			self.active_signature = (self.active_signature + 1) % self.signatures.len();
		}
	}

	/// Cycles to the previous signature (for overloads).
	pub fn prev_signature(&mut self) {
		if self.signatures.len() > 1 {
			if self.active_signature == 0 {
				self.active_signature = self.signatures.len() - 1;
			} else {
				self.active_signature -= 1;
			}
		}
	}

	/// Calculates the content size for the popup.
	fn content_size(&self) -> (u16, u16) {
		let sig = match self.active_signature() {
			Some(s) => s,
			None => return (20, 3),
		};

		// Calculate width based on signature label
		let sig_width = sig.label.len() as u16;

		// Check if we have documentation to show
		let has_doc = sig.documentation.is_some();
		let doc_lines = if has_doc {
			// Count lines in documentation (simplified)
			self.doc_line_count()
		} else {
			0
		};

		// Add line for overload indicator if multiple signatures
		let overload_line = if self.signatures.len() > 1 { 1 } else { 0 };

		// Total height: signature + separator + docs + overload indicator + border
		let height = 1 + doc_lines + overload_line + 2;

		(sig_width.max(20) + 2, (height as u16).min(MAX_HEIGHT))
	}

	/// Counts the number of lines in the documentation.
	fn doc_line_count(&self) -> usize {
		let sig = match self.active_signature() {
			Some(s) => s,
			None => return 0,
		};

		match &sig.documentation {
			Some(Documentation::String(s)) => s.lines().count().min(3),
			Some(Documentation::MarkupContent(m)) => m.value.lines().count().min(3),
			None => 0,
		}
	}

	/// Extracts documentation text from a signature.
	fn get_doc_text(&self) -> Option<String> {
		let sig = self.active_signature()?;
		match &sig.documentation {
			Some(Documentation::String(s)) => Some(s.clone()),
			Some(Documentation::MarkupContent(m)) => Some(strip_markdown(&m.value)),
			None => None,
		}
	}
}

impl Popup for SignaturePopup {
	fn id(&self) -> &str {
		"lsp-signature"
	}

	fn anchor(&self) -> PopupAnchor {
		self.anchor
	}

	fn size_hints(&self) -> SizeHints {
		let (width, height) = self.content_size();
		SizeHints {
			min_width: 20,
			min_height: 3,
			max_width: MAX_WIDTH,
			max_height: MAX_HEIGHT,
			preferred_width: width.min(MAX_WIDTH),
			preferred_height: height,
		}
	}

	fn handle_event(&mut self, event: PopupEvent) -> PopupEventResult {
		match event {
			PopupEvent::Key(key) => self.handle_key(key),
			PopupEvent::Mouse(mouse) => match mouse.kind {
				MouseEventKind::ScrollUp => {
					self.prev_signature();
					PopupEventResult::consumed()
				}
				MouseEventKind::ScrollDown => {
					self.next_signature();
					PopupEventResult::consumed()
				}
				_ => PopupEventResult::consumed(),
			},
			// Don't dismiss on cursor movement - stay open while typing
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

		// Render content inside the border
		let content_area = Rect::new(
			area.x + 1,
			area.y + 1,
			area.width.saturating_sub(2),
			area.height.saturating_sub(2),
		);

		if content_area.width > 0 && content_area.height > 0 {
			self.render_content(&mut buffer, content_area, theme);
		}

		// Merge buffer into frame
		frame.render_widget(BufferWidget(buffer), area);
	}

	fn is_modal(&self) -> bool {
		false
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

impl SignaturePopup {
	/// Handles key events for the signature popup.
	fn handle_key(&mut self, key: KeyEvent) -> PopupEventResult {
		match (key.code, key.modifiers) {
			// Cycle through overloads
			(KeyCode::Char('n'), Modifiers::CONTROL) | (KeyCode::Down, _) => {
				self.next_signature();
				PopupEventResult::consumed()
			}
			(KeyCode::Char('p'), Modifiers::CONTROL) | (KeyCode::Up, _) => {
				self.prev_signature();
				PopupEventResult::consumed()
			}

			// Escape dismisses
			(KeyCode::Escape, _) => PopupEventResult::dismissed(),

			// Let other keys through (for continued typing)
			_ => PopupEventResult::not_consumed(),
		}
	}

	/// Renders the signature content.
	fn render_content(&self, buffer: &mut Buffer, area: Rect, theme: &Theme) {
		let Some(sig) = self.active_signature() else {
			return;
		};

		let mut y_offset = 0;

		// Render signature with highlighted parameter
		if y_offset < area.height {
			self.render_signature_line(buffer, area, sig, y_offset, theme);
			y_offset += 1;
		}

		// Render overload indicator if multiple signatures
		if self.signatures.len() > 1 && y_offset < area.height {
			let indicator = format!("{}/{}", self.active_signature + 1, self.signatures.len());
			let indicator_style = Style::default()
				.fg(theme.colors.popup.border)
				.add_modifier(Modifier::DIM);

			let indicator_area = Rect::new(area.x, area.y + y_offset, area.width, 1);
			Line::from(Span::styled(indicator, indicator_style)).render(indicator_area, buffer);
			y_offset += 1;
		}

		// Render documentation if present and there's space
		if let Some(doc) = self.get_doc_text() {
			if y_offset < area.height {
				// Draw separator
				for x in area.x..area.x + area.width {
					if let Some(cell) = buffer.cell_mut((x, area.y + y_offset)) {
						cell.set_symbol("─")
							.set_fg(theme.colors.popup.border)
							.set_bg(theme.colors.popup.bg);
					}
				}
				y_offset += 1;
			}

			let doc_style = Style::default()
				.fg(theme.colors.popup.fg)
				.add_modifier(Modifier::DIM);

			for line in doc.lines().take((area.height - y_offset) as usize) {
				if y_offset >= area.height {
					break;
				}
				let truncated: String = line.chars().take(area.width as usize).collect();
				let line_area = Rect::new(area.x, area.y + y_offset, area.width, 1);
				Line::from(Span::styled(truncated, doc_style)).render(line_area, buffer);
				y_offset += 1;
			}
		}
	}

	/// Renders the signature line with the active parameter highlighted.
	fn render_signature_line(
		&self,
		buffer: &mut Buffer,
		area: Rect,
		sig: &SignatureInformation,
		y_offset: u16,
		theme: &Theme,
	) {
		let normal_style = Style::default().fg(theme.colors.popup.fg);
		let highlight_style = Style::default()
			.fg(Color::Yellow)
			.add_modifier(Modifier::BOLD);

		// Find parameter range to highlight
		let highlight_range = self.find_parameter_range(sig);

		let line_area = Rect::new(area.x, area.y + y_offset, area.width, 1);

		// Truncate if needed
		let label = &sig.label;
		let display_len = label.len().min(area.width as usize);
		let display_label: String = label.chars().take(display_len).collect();

		if let Some((start, end)) = highlight_range {
			// Render with highlighted parameter
			let start = start.min(display_len);
			let end = end.min(display_len);

			let mut spans = Vec::new();

			if start > 0 {
				spans.push(Span::styled(&display_label[..start], normal_style));
			}

			if start < end {
				spans.push(Span::styled(&display_label[start..end], highlight_style));
			}

			if end < display_len {
				spans.push(Span::styled(&display_label[end..], normal_style));
			}

			Line::from(spans).render(line_area, buffer);
		} else {
			// No parameter to highlight
			Line::from(Span::styled(display_label, normal_style)).render(line_area, buffer);
		}
	}

	/// Finds the byte range of the active parameter in the signature label.
	fn find_parameter_range(&self, sig: &SignatureInformation) -> Option<(usize, usize)> {
		let active_param = self.active_parameter?;
		let params = sig.parameters.as_ref()?;
		let param = params.get(active_param)?;

		match &param.label {
			ParameterLabel::Simple(name) => {
				// Find the parameter name in the signature label
				sig.label
					.find(name.as_str())
					.map(|start| (start, start + name.len()))
			}
			ParameterLabel::LabelOffsets(offsets) => {
				Some((offsets[0] as usize, offsets[1] as usize))
			}
		}
	}
}

/// Strips basic markdown formatting from text.
fn strip_markdown(text: &str) -> String {
	let mut result = String::new();
	let mut in_code_block = false;

	for line in text.lines() {
		if line.starts_with("```") {
			in_code_block = !in_code_block;
			continue;
		}

		if in_code_block {
			result.push_str(line);
			result.push('\n');
			continue;
		}

		// Remove headers
		let line = line.trim_start_matches('#').trim_start();

		// Remove bold/italic markers
		let line = line.replace("**", "").replace("__", "");

		// Remove inline code backticks
		let line = line.replace('`', "");

		result.push_str(&line);
		result.push('\n');
	}

	result.trim_end().to_string()
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

	fn make_signature(label: &str, params: Vec<&str>) -> SignatureInformation {
		SignatureInformation {
			label: label.to_string(),
			documentation: None,
			parameters: Some(
				params
					.into_iter()
					.map(|p| ParameterInformation {
						label: ParameterLabel::Simple(p.to_string()),
						documentation: None,
					})
					.collect(),
			),
			active_parameter: None,
		}
	}

	#[test]
	fn test_signature_popup_creation() {
		let help = SignatureHelp {
			signatures: vec![make_signature(
				"fn foo(a: i32, b: String)",
				vec!["a: i32", "b: String"],
			)],
			active_signature: Some(0),
			active_parameter: Some(0),
		};

		let popup = SignaturePopup::from_signature_help(help).unwrap();
		assert_eq!(popup.id(), "lsp-signature");
		assert_eq!(popup.signature_count(), 1);
		assert!(!popup.is_modal());
	}

	#[test]
	fn test_signature_popup_empty() {
		let help = SignatureHelp {
			signatures: vec![],
			active_signature: None,
			active_parameter: None,
		};

		assert!(SignaturePopup::from_signature_help(help).is_none());
	}

	#[test]
	fn test_cycle_signatures() {
		let help = SignatureHelp {
			signatures: vec![
				make_signature("fn foo(a: i32)", vec!["a: i32"]),
				make_signature("fn foo(a: i32, b: i32)", vec!["a: i32", "b: i32"]),
				make_signature(
					"fn foo(a: i32, b: i32, c: i32)",
					vec!["a: i32", "b: i32", "c: i32"],
				),
			],
			active_signature: Some(0),
			active_parameter: None,
		};

		let mut popup = SignaturePopup::from_signature_help(help).unwrap();
		assert_eq!(popup.active_signature_index(), 0);

		popup.next_signature();
		assert_eq!(popup.active_signature_index(), 1);

		popup.next_signature();
		assert_eq!(popup.active_signature_index(), 2);

		popup.next_signature(); // Wraps around
		assert_eq!(popup.active_signature_index(), 0);

		popup.prev_signature(); // Wraps backward
		assert_eq!(popup.active_signature_index(), 2);
	}

	#[test]
	fn test_parameter_highlight_simple() {
		let sig = make_signature("fn foo(a: i32, b: String)", vec!["a: i32", "b: String"]);

		let help = SignatureHelp {
			signatures: vec![sig],
			active_signature: Some(0),
			active_parameter: Some(0),
		};

		let popup = SignaturePopup::from_signature_help(help).unwrap();
		let sig = popup.active_signature().unwrap();
		let range = popup.find_parameter_range(sig);

		// Should find "a: i32" in the signature
		assert!(range.is_some());
		let (start, end) = range.unwrap();
		assert_eq!(&sig.label[start..end], "a: i32");
	}

	#[test]
	fn test_parameter_highlight_offsets() {
		let sig = SignatureInformation {
			label: "fn bar(x: u8, y: u8)".to_string(),
			documentation: None,
			parameters: Some(vec![
				ParameterInformation {
					label: ParameterLabel::LabelOffsets([7, 12]),
					documentation: None,
				},
				ParameterInformation {
					label: ParameterLabel::LabelOffsets([14, 19]),
					documentation: None,
				},
			]),
			active_parameter: None,
		};

		let help = SignatureHelp {
			signatures: vec![sig],
			active_signature: Some(0),
			active_parameter: Some(1), // Second parameter
		};

		let popup = SignaturePopup::from_signature_help(help).unwrap();
		let sig = popup.active_signature().unwrap();
		let range = popup.find_parameter_range(sig);

		assert!(range.is_some());
		let (start, end) = range.unwrap();
		assert_eq!(&sig.label[start..end], "y: u8");
	}

	#[test]
	fn test_size_hints() {
		let help = SignatureHelp {
			signatures: vec![make_signature("fn foo(a: i32)", vec!["a: i32"])],
			active_signature: Some(0),
			active_parameter: None,
		};

		let popup = SignaturePopup::from_signature_help(help).unwrap();
		let hints = popup.size_hints();

		assert!(hints.min_width >= 20);
		assert!(hints.min_height >= 3);
		assert!(hints.max_width <= MAX_WIDTH);
		assert!(hints.max_height <= MAX_HEIGHT);
	}

	#[test]
	fn test_strip_markdown() {
		let markdown = "# Header\n**bold** text\n`code`\n```rust\nfn foo() {}\n```";
		let stripped = strip_markdown(markdown);

		assert!(!stripped.contains('#'));
		assert!(!stripped.contains("**"));
		assert!(!stripped.contains('`'));
		assert!(stripped.contains("fn foo()"));
	}
}
