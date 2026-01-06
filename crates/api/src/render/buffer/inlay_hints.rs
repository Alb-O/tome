//! Inlay hints rendering utilities.
//!
//! Converts LSP inlay hints to display-ready structures for rendering
//! as virtual text inline with code (type annotations, parameter names, etc.).

use std::collections::HashMap;

use ropey::Rope;
use xeno_lsp::lsp_types::{InlayHint, InlayHintKind, InlayHintLabel};
use xeno_lsp::{lsp_position_to_char, OffsetEncoding};

/// Display-ready inlay hint for a specific position.
#[derive(Debug, Clone)]
pub struct InlayHintDisplay {
	/// Character position in document where the hint appears.
	pub char_pos: usize,
	/// Line number (0-indexed).
	pub line: usize,
	/// Column offset within the line (char offset).
	pub column: usize,
	/// The text to display.
	pub text: String,
	/// The kind of hint (Type, Parameter, or None/unknown).
	pub kind: InlayHintDisplayKind,
	/// Whether to add a space before the hint text.
	pub padding_left: bool,
	/// Whether to add a space after the hint text.
	pub padding_right: bool,
}

/// Kind of inlay hint for styling purposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlayHintDisplayKind {
	/// Type annotation (e.g., `: i32` after a variable).
	Type,
	/// Parameter name (e.g., `name:` before an argument).
	Parameter,
	/// Unknown or unspecified kind.
	Other,
}

impl From<Option<InlayHintKind>> for InlayHintDisplayKind {
	fn from(kind: Option<InlayHintKind>) -> Self {
		match kind {
			Some(InlayHintKind::TYPE) => InlayHintDisplayKind::Type,
			Some(InlayHintKind::PARAMETER) => InlayHintDisplayKind::Parameter,
			_ => InlayHintDisplayKind::Other,
		}
	}
}

/// Per-line inlay hints for efficient rendering.
#[derive(Debug, Clone, Default)]
pub struct LineInlayHints {
	/// Hints on this line, sorted by column position.
	pub hints: Vec<InlayHintDisplay>,
}

/// Prepared inlay hints for a buffer, organized for efficient rendering.
#[derive(Debug, Clone, Default)]
pub struct PreparedInlayHints {
	/// All hints in document order.
	pub all: Vec<InlayHintDisplay>,
	/// Hints grouped by line number for fast per-line lookup.
	pub by_line: HashMap<usize, LineInlayHints>,
}

impl PreparedInlayHints {
	/// Returns the inlay hints for a given line, if any.
	pub fn line(&self, line_idx: usize) -> Option<&LineInlayHints> {
		self.by_line.get(&line_idx)
	}

	/// Returns all hints at or after a given character position on a line.
	pub fn hints_after(&self, line_idx: usize, column: usize) -> impl Iterator<Item = &InlayHintDisplay> {
		self.by_line
			.get(&line_idx)
			.into_iter()
			.flat_map(move |lh| lh.hints.iter().filter(move |h| h.column >= column))
	}

	/// Returns true if there are any inlay hints.
	pub fn is_empty(&self) -> bool {
		self.all.is_empty()
	}

	/// Returns the total number of inlay hints.
	pub fn len(&self) -> usize {
		self.all.len()
	}
}

/// Extract the display text from an InlayHintLabel.
fn label_text(label: &InlayHintLabel) -> String {
	match label {
		InlayHintLabel::String(s) => s.clone(),
		InlayHintLabel::LabelParts(parts) => parts.iter().map(|p| p.value.as_str()).collect(),
	}
}

/// Prepare LSP inlay hints for display.
///
/// Converts LSP `InlayHint` objects into display-ready `InlayHintDisplay` structs,
/// handling position encoding and organizing by line.
pub fn prepare_inlay_hints(
	content: &Rope,
	hints: &[InlayHint],
	encoding: OffsetEncoding,
) -> PreparedInlayHints {
	let mut all = Vec::with_capacity(hints.len());
	let mut by_line: HashMap<usize, LineInlayHints> = HashMap::new();

	for hint in hints {
		// Convert LSP position to char position
		let Some(char_pos) = lsp_position_to_char(content, hint.position, encoding) else {
			continue;
		};

		// Get line and column info
		let line = content.char_to_line(char_pos);
		let line_start = content.line_to_char(line);
		let column = char_pos - line_start;

		let text = label_text(&hint.label);

		let display = InlayHintDisplay {
			char_pos,
			line,
			column,
			text,
			kind: hint.kind.into(),
			padding_left: hint.padding_left.unwrap_or(false),
			padding_right: hint.padding_right.unwrap_or(false),
		};

		// Add to by_line map
		let line_hints = by_line.entry(line).or_default();
		line_hints.hints.push(display.clone());

		all.push(display);
	}

	// Sort hints within each line by column position
	for line_hints in by_line.values_mut() {
		line_hints.hints.sort_by_key(|h| h.column);
	}

	// Sort all hints by position
	all.sort_by_key(|h| (h.line, h.column));

	PreparedInlayHints { all, by_line }
}

#[cfg(test)]
mod tests {
	use super::*;
	use xeno_lsp::lsp_types::Position;

	fn make_hint(line: u32, character: u32, label: &str, kind: Option<InlayHintKind>) -> InlayHint {
		InlayHint {
			position: Position { line, character },
			label: InlayHintLabel::String(label.to_string()),
			kind,
			text_edits: None,
			tooltip: None,
			padding_left: Some(true),
			padding_right: Some(false),
			data: None,
		}
	}

	#[test]
	fn test_prepare_single_hint() {
		let content = Rope::from("let x = 42;\n");
		let hints = vec![make_hint(0, 5, ": i32", Some(InlayHintKind::TYPE))];

		let prepared = prepare_inlay_hints(&content, &hints, OffsetEncoding::Utf32);

		assert_eq!(prepared.len(), 1);
		assert!(!prepared.is_empty());

		let hint = &prepared.all[0];
		assert_eq!(hint.line, 0);
		assert_eq!(hint.column, 5);
		assert_eq!(hint.text, ": i32");
		assert_eq!(hint.kind, InlayHintDisplayKind::Type);
		assert!(hint.padding_left);
		assert!(!hint.padding_right);

		let line_hints = prepared.line(0).unwrap();
		assert_eq!(line_hints.hints.len(), 1);
	}

	#[test]
	fn test_multiple_hints_same_line() {
		let content = Rope::from("fn foo(a, b, c) {}\n");
		let hints = vec![
			make_hint(0, 7, "x:", Some(InlayHintKind::PARAMETER)),
			make_hint(0, 10, "y:", Some(InlayHintKind::PARAMETER)),
			make_hint(0, 13, "z:", Some(InlayHintKind::PARAMETER)),
		];

		let prepared = prepare_inlay_hints(&content, &hints, OffsetEncoding::Utf32);

		assert_eq!(prepared.len(), 3);

		let line_hints = prepared.line(0).unwrap();
		assert_eq!(line_hints.hints.len(), 3);

		// Verify they're sorted by column
		assert_eq!(line_hints.hints[0].column, 7);
		assert_eq!(line_hints.hints[1].column, 10);
		assert_eq!(line_hints.hints[2].column, 13);
	}

	#[test]
	fn test_hints_across_lines() {
		let content = Rope::from("let a = 1;\nlet b = 2;\n");
		let hints = vec![
			make_hint(0, 5, ": i32", Some(InlayHintKind::TYPE)),
			make_hint(1, 5, ": i32", Some(InlayHintKind::TYPE)),
		];

		let prepared = prepare_inlay_hints(&content, &hints, OffsetEncoding::Utf32);

		assert_eq!(prepared.len(), 2);
		assert!(prepared.line(0).is_some());
		assert!(prepared.line(1).is_some());
		assert!(prepared.line(2).is_none());
	}

	#[test]
	fn test_hint_kind_conversion() {
		assert_eq!(
			InlayHintDisplayKind::from(Some(InlayHintKind::TYPE)),
			InlayHintDisplayKind::Type
		);
		assert_eq!(
			InlayHintDisplayKind::from(Some(InlayHintKind::PARAMETER)),
			InlayHintDisplayKind::Parameter
		);
		assert_eq!(
			InlayHintDisplayKind::from(None),
			InlayHintDisplayKind::Other
		);
	}
}
