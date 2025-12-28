//! Word wrapping line composer.
use alloc::collections::VecDeque;
use alloc::vec;
use alloc::vec::Vec;
use core::mem;

use super::{LineComposer, WrappedLine};
use crate::layout::Alignment;
use crate::text::StyledGrapheme;

/// A state machine that wraps lines on word boundaries.
#[derive(Debug, Default, Clone)]
pub struct WordWrapper<'a, O, I>
where
	// Outer iterator providing the individual lines
	O: Iterator<Item = (I, Alignment)>,
	// Inner iterator providing the styled symbols of a line Each line consists of an alignment and
	// a series of symbols
	I: Iterator<Item = StyledGrapheme<'a>>,
{
	/// The given, unprocessed lines
	input_lines: O,
	max_line_width: u16,
	wrapped_lines: VecDeque<Vec<StyledGrapheme<'a>>>,
	current_alignment: Alignment,
	current_line: Vec<StyledGrapheme<'a>>,
	/// Removes the leading whitespace from lines
	trim: bool,

	// These are cached allocations that hold no state across next_line invocations
	pending_word: Vec<StyledGrapheme<'a>>,
	pending_whitespace: VecDeque<StyledGrapheme<'a>>,
	pending_line_pool: Vec<Vec<StyledGrapheme<'a>>>,
}

impl<'a, O, I> WordWrapper<'a, O, I>
where
	O: Iterator<Item = (I, Alignment)>,
	I: Iterator<Item = StyledGrapheme<'a>>,
{
	/// Create a new `WordWrapper` with the given lines and maximum line width.
	pub const fn new(lines: O, max_line_width: u16, trim: bool) -> Self {
		Self {
			input_lines: lines,
			max_line_width,
			wrapped_lines: VecDeque::new(),
			current_alignment: Alignment::Left,
			current_line: vec![],
			trim,

			pending_word: Vec::new(),
			pending_line_pool: Vec::new(),
			pending_whitespace: VecDeque::new(),
		}
	}

	/// Split an input line (`line_symbols`) into wrapped lines
	/// and cache them to be emitted later
	fn process_input(&mut self, line_symbols: impl IntoIterator<Item = StyledGrapheme<'a>>) {
		use unicode_width::UnicodeWidthStr;

		let mut pending_line = self.pending_line_pool.pop().unwrap_or_default();
		let mut line_width = 0;
		let mut word_width = 0;
		let mut whitespace_width = 0;
		let mut non_whitespace_previous = false;

		self.pending_word.clear();
		self.pending_whitespace.clear();
		pending_line.clear();

		for grapheme in line_symbols {
			let is_whitespace = grapheme.is_whitespace();
			let symbol_width = grapheme.symbol.width() as u16;

			// ignore symbols wider than line limit
			if symbol_width > self.max_line_width {
				continue;
			}

			let word_found = non_whitespace_previous && is_whitespace;
			// current word would overflow after removing whitespace
			let trimmed_overflow = pending_line.is_empty()
				&& self.trim && word_width + symbol_width > self.max_line_width;
			// separated whitespace would overflow on its own
			let whitespace_overflow = pending_line.is_empty()
				&& self.trim && whitespace_width + symbol_width
				> self.max_line_width;
			// current full word (including whitespace) would overflow
			let untrimmed_overflow = pending_line.is_empty()
				&& !self.trim
				&& word_width + whitespace_width + symbol_width > self.max_line_width;

			// append finished segment to current line
			if word_found || trimmed_overflow || whitespace_overflow || untrimmed_overflow {
				if !pending_line.is_empty() || !self.trim {
					pending_line.extend(self.pending_whitespace.drain(..));
					line_width += whitespace_width;
				}

				pending_line.append(&mut self.pending_word);
				line_width += word_width;

				self.pending_whitespace.clear();
				whitespace_width = 0;
				word_width = 0;
			}

			// pending line fills up limit
			let line_full = line_width >= self.max_line_width;
			// pending word would overflow line limit
			let pending_word_overflow = symbol_width > 0
				&& line_width + whitespace_width + word_width >= self.max_line_width;

			// add finished wrapped line to remaining lines
			if line_full || pending_word_overflow {
				let mut remaining_width = u16::saturating_sub(self.max_line_width, line_width);

				self.wrapped_lines.push_back(mem::take(&mut pending_line));
				line_width = 0;

				// remove whitespace up to the end of line
				while let Some(grapheme) = self.pending_whitespace.front() {
					let width = grapheme.symbol.width() as u16;

					if width > remaining_width {
						break;
					}

					whitespace_width -= width;
					remaining_width -= width;
					self.pending_whitespace.pop_front();
				}

				// don't count first whitespace toward next word
				if is_whitespace && self.pending_whitespace.is_empty() {
					continue;
				}
			}

			// append symbol to a pending buffer
			if is_whitespace {
				whitespace_width += symbol_width;
				self.pending_whitespace.push_back(grapheme);
			} else {
				word_width += symbol_width;
				self.pending_word.push(grapheme);
			}

			non_whitespace_previous = !is_whitespace;
		}

		// append remaining text parts
		if pending_line.is_empty()
			&& self.pending_word.is_empty()
			&& !self.pending_whitespace.is_empty()
			&& self.trim
		{
			self.wrapped_lines.push_back(vec![]);
		}
		if !pending_line.is_empty() || !self.trim {
			pending_line.extend(self.pending_whitespace.drain(..));
		}
		pending_line.append(&mut self.pending_word);

		#[expect(clippy::else_if_without_else)]
		if !pending_line.is_empty() {
			self.wrapped_lines.push_back(pending_line);
		} else if pending_line.capacity() > 0 {
			self.pending_line_pool.push(pending_line);
		}
		if self.wrapped_lines.is_empty() {
			self.wrapped_lines.push_back(vec![]);
		}
	}

	fn replace_current_line(&mut self, line: Vec<StyledGrapheme<'a>>) {
		let cache = mem::replace(&mut self.current_line, line);
		if cache.capacity() > 0 {
			self.pending_line_pool.push(cache);
		}
	}
}

impl<'a, O, I> LineComposer<'a> for WordWrapper<'a, O, I>
where
	O: Iterator<Item = (I, Alignment)>,
	I: Iterator<Item = StyledGrapheme<'a>>,
{
	fn next_line<'lend>(&'lend mut self) -> Option<WrappedLine<'lend, 'a>> {
		use unicode_width::UnicodeWidthStr;

		if self.max_line_width == 0 {
			return None;
		}

		loop {
			// emit next cached line if present
			if let Some(line) = self.wrapped_lines.pop_front() {
				let line_width = line
					.iter()
					.map(|grapheme| grapheme.symbol.width() as u16)
					.sum();

				self.replace_current_line(line);
				return Some(WrappedLine {
					graphemes: &self.current_line,
					width: line_width,
					alignment: self.current_alignment,
				});
			}

			// otherwise, process pending wrapped lines from input
			let (line_symbols, line_alignment) = self.input_lines.next()?;
			self.current_alignment = line_alignment;
			self.process_input(line_symbols);
		}
	}
}
