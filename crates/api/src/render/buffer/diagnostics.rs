//! Diagnostic rendering utilities.
//!
//! Converts LSP diagnostics to display-ready structures for rendering
//! in the buffer (underlines, gutter signs, virtual text).

use std::collections::HashMap;

use ropey::Rope;
use xeno_lsp::lsp_types::DiagnosticSeverity;
use xeno_lsp::{OffsetEncoding, lsp_range_to_char_range, lsp_types::Diagnostic};

/// Display-ready diagnostic information for a single diagnostic.
#[derive(Debug, Clone)]
pub struct DiagnosticDisplay {
	/// 0-indexed line number.
	pub line: usize,
	/// Start column (char offset within line).
	pub start_col: usize,
	/// End column (char offset within line).
	pub end_col: usize,
	/// Character range in document (start).
	pub char_start: usize,
	/// Character range in document (end).
	pub char_end: usize,
	/// Severity level (1=error, 2=warning, 3=info, 4=hint per LSP spec).
	pub severity: u8,
	/// Diagnostic message.
	pub message: String,
	/// Source of the diagnostic (e.g., "rustc", "clippy").
	pub source: Option<String>,
}

impl DiagnosticDisplay {
	/// Returns the gutter severity value (4=error, 3=warn, 2=info, 1=hint).
	///
	/// This inverts the LSP severity for gutter display where higher = more severe.
	pub fn gutter_severity(&self) -> u8 {
		match self.severity {
			1 => 4, // Error -> 4
			2 => 3, // Warning -> 3
			3 => 2, // Info -> 2
			4 => 1, // Hint -> 1
			_ => 0, // Unknown -> 0
		}
	}
}

/// Per-line diagnostic summary for gutter and virtual text rendering.
#[derive(Debug, Clone, Default)]
pub struct LineDiagnostics {
	/// Highest severity on this line (gutter format: 4=error, 3=warn, 2=info, 1=hint).
	pub max_severity: u8,
	/// All diagnostics on this line.
	pub diagnostics: Vec<DiagnosticDisplay>,
}

impl LineDiagnostics {
	/// Returns the most severe diagnostic message for virtual text display.
	pub fn first_message(&self) -> Option<&str> {
		self.diagnostics.first().map(|d| d.message.as_str())
	}

	/// Returns whether this line has any error-level diagnostics.
	pub fn has_error(&self) -> bool {
		self.max_severity >= 4
	}

	/// Returns whether this line has any warning-level diagnostics.
	pub fn has_warning(&self) -> bool {
		self.max_severity >= 3
	}
}

/// Prepared diagnostics for a buffer, organized for efficient rendering.
#[derive(Debug, Clone, Default)]
pub struct PreparedDiagnostics {
	/// All diagnostics in document order.
	pub all: Vec<DiagnosticDisplay>,
	/// Diagnostics grouped by line number.
	pub by_line: HashMap<usize, LineDiagnostics>,
}

impl PreparedDiagnostics {
	/// Returns the line diagnostics for a given line, if any.
	pub fn line(&self, line_idx: usize) -> Option<&LineDiagnostics> {
		self.by_line.get(&line_idx)
	}

	/// Returns the gutter severity for a line (4=error, 3=warn, 2=info, 1=hint, 0=none).
	pub fn gutter_severity(&self, line_idx: usize) -> u8 {
		self.by_line
			.get(&line_idx)
			.map(|ld| ld.max_severity)
			.unwrap_or(0)
	}

	/// Returns true if there are any diagnostics.
	pub fn is_empty(&self) -> bool {
		self.all.is_empty()
	}

	/// Returns the total number of diagnostics.
	pub fn len(&self) -> usize {
		self.all.len()
	}

	/// Returns error count.
	pub fn error_count(&self) -> usize {
		self.all.iter().filter(|d| d.severity == 1).count()
	}

	/// Returns warning count.
	pub fn warning_count(&self) -> usize {
		self.all.iter().filter(|d| d.severity == 2).count()
	}

	/// Returns the diagnostic severity at a given character position.
	///
	/// If multiple diagnostics overlap, returns the most severe one.
	/// Severity uses LSP values: 1=Error, 2=Warning, 3=Info, 4=Hint.
	/// Returns 0 if no diagnostic covers this position.
	pub fn severity_at_char(&self, char_pos: usize) -> u8 {
		let mut most_severe: u8 = 0;
		for diag in &self.all {
			if char_pos >= diag.char_start && char_pos < diag.char_end {
				// Lower severity number = more severe (1=error is most severe)
				if most_severe == 0 || diag.severity < most_severe {
					most_severe = diag.severity;
				}
			}
		}
		most_severe
	}
}

/// Prepare LSP diagnostics for display.
///
/// Converts LSP `Diagnostic` objects into display-ready `DiagnosticDisplay` structs,
/// handling position encoding and organizing by line.
pub fn prepare_diagnostics(
	content: &Rope,
	diagnostics: &[Diagnostic],
	encoding: OffsetEncoding,
) -> PreparedDiagnostics {
	let mut all = Vec::with_capacity(diagnostics.len());
	let mut by_line: HashMap<usize, LineDiagnostics> = HashMap::new();

	for diag in diagnostics {
		// Convert LSP range to char range
		let Some((char_start, char_end)) =
			lsp_range_to_char_range(content, diag.range, encoding)
		else {
			continue;
		};

		// Get line and column info
		let line = content.char_to_line(char_start);
		let line_start_char = content.line_to_char(line);
		let start_col = char_start - line_start_char;

		// End column - handle multi-line diagnostics by clamping to line end
		let line_end_char = if line + 1 < content.len_lines() {
			content.line_to_char(line + 1)
		} else {
			content.len_chars()
		};
		let end_col = if char_end > line_end_char {
			line_end_char - line_start_char
		} else {
			char_end - line_start_char
		};

		// LSP severity: 1=Error, 2=Warning, 3=Information, 4=Hint
		let severity = match diag.severity {
			Some(DiagnosticSeverity::ERROR) => 1,
			Some(DiagnosticSeverity::WARNING) => 2,
			Some(DiagnosticSeverity::INFORMATION) => 3,
			Some(DiagnosticSeverity::HINT) => 4,
			_ => 1, // Default to error if unknown
		};

		let display = DiagnosticDisplay {
			line,
			start_col,
			end_col,
			char_start,
			char_end,
			severity,
			message: diag.message.clone(),
			source: diag.source.clone(),
		};

		let gutter_sev = display.gutter_severity();

		// Add to by_line map
		let line_diags = by_line.entry(line).or_default();
		if gutter_sev > line_diags.max_severity {
			line_diags.max_severity = gutter_sev;
		}
		line_diags.diagnostics.push(display.clone());

		all.push(display);
	}

	// Sort diagnostics within each line by severity (most severe first)
	for line_diags in by_line.values_mut() {
		line_diags
			.diagnostics
			.sort_by(|a, b| a.severity.cmp(&b.severity));
	}

	// Sort all diagnostics by position
	all.sort_by_key(|d| (d.line, d.start_col));

	PreparedDiagnostics { all, by_line }
}

#[cfg(test)]
mod tests {
	use super::*;
	use xeno_lsp::lsp_types::{DiagnosticSeverity, Position, Range};

	fn make_diagnostic(
		start_line: u32,
		start_char: u32,
		end_line: u32,
		end_char: u32,
		severity: DiagnosticSeverity,
		message: &str,
	) -> Diagnostic {
		Diagnostic {
			range: Range {
				start: Position {
					line: start_line,
					character: start_char,
				},
				end: Position {
					line: end_line,
					character: end_char,
				},
			},
			severity: Some(severity),
			message: message.to_string(),
			..Default::default()
		}
	}

	#[test]
	fn test_prepare_single_diagnostic() {
		let content = Rope::from("fn main() {\n    let x = 42;\n}\n");
		let diagnostics = vec![make_diagnostic(
			1,
			8,
			1,
			9,
			DiagnosticSeverity::WARNING,
			"unused variable",
		)];

		let prepared = prepare_diagnostics(&content, &diagnostics, OffsetEncoding::Utf32);

		assert_eq!(prepared.len(), 1);
		assert_eq!(prepared.error_count(), 0);
		assert_eq!(prepared.warning_count(), 1);

		let diag = &prepared.all[0];
		assert_eq!(diag.line, 1);
		assert_eq!(diag.start_col, 8);
		assert_eq!(diag.end_col, 9);
		assert_eq!(diag.severity, 2); // Warning
		assert_eq!(diag.gutter_severity(), 3); // Inverted for gutter

		let line_diags = prepared.line(1).unwrap();
		assert_eq!(line_diags.max_severity, 3);
		assert!(!line_diags.has_error());
		assert!(line_diags.has_warning());
	}

	#[test]
	fn test_multiple_diagnostics_same_line() {
		let content = Rope::from("let x = y + z;\n");
		let diagnostics = vec![
			make_diagnostic(0, 4, 0, 5, DiagnosticSeverity::ERROR, "undefined: y"),
			make_diagnostic(0, 8, 0, 9, DiagnosticSeverity::ERROR, "undefined: z"),
		];

		let prepared = prepare_diagnostics(&content, &diagnostics, OffsetEncoding::Utf32);

		assert_eq!(prepared.len(), 2);
		assert_eq!(prepared.error_count(), 2);

		let line_diags = prepared.line(0).unwrap();
		assert_eq!(line_diags.max_severity, 4); // Error
		assert_eq!(line_diags.diagnostics.len(), 2);
		assert!(line_diags.has_error());
	}

	#[test]
	fn test_gutter_severity_mapping() {
		// LSP: 1=Error, 2=Warning, 3=Info, 4=Hint
		// Gutter: 4=Error, 3=Warning, 2=Info, 1=Hint
		let diag = DiagnosticDisplay {
			line: 0,
			start_col: 0,
			end_col: 1,
			char_start: 0,
			char_end: 1,
			severity: 1, // Error
			message: String::new(),
			source: None,
		};
		assert_eq!(diag.gutter_severity(), 4);

		let diag = DiagnosticDisplay {
			severity: 4, // Hint
			..diag
		};
		assert_eq!(diag.gutter_severity(), 1);
	}
}
