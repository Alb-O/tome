//! A widget for displaying key bindings with tree-style connectors.
//!
//! [`KeyTree`] renders a root key with child continuations using
//! box-drawing characters.
//!
//! # Example
//!
//! ```
//! use evildoer_tui::widgets::keytree::{KeyTree, KeyTreeNode};
//!
//! let children = vec![
//!     KeyTreeNode::new("g", "document_start"),
//!     KeyTreeNode::new("e", "document_end"),
//! ];
//! let tree = KeyTree::new("g", children);
//! ```

use alloc::borrow::Cow;
use alloc::vec::Vec;

use crate::buffer::Buffer;
use crate::layout::Rect;
use crate::style::Style;
use crate::symbols::line;
use crate::widgets::Widget;

/// A key binding entry with its description.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyTreeNode<'a> {
	/// The key (e.g., "g", "space").
	pub key: Cow<'a, str>,
	/// Description of the action.
	pub description: Cow<'a, str>,
}

impl<'a> KeyTreeNode<'a> {
	/// Creates a new node.
	pub fn new(key: impl Into<Cow<'a, str>>, description: impl Into<Cow<'a, str>>) -> Self {
		Self { key: key.into(), description: description.into() }
	}
}

/// Line symbols for tree connectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TreeSymbols<'a> {
	/// Branch connector (middle items).
	pub branch: &'a str,
	/// Corner connector (last item).
	pub corner: &'a str,
	/// Horizontal line after connector.
	pub horizontal: &'a str,
}

impl Default for TreeSymbols<'_> {
	fn default() -> Self {
		ROUNDED_SYMBOLS
	}
}

/// Rounded tree symbols (default).
pub const ROUNDED_SYMBOLS: TreeSymbols<'static> = TreeSymbols {
	branch: line::VERTICAL_RIGHT,
	corner: line::ROUNDED_BOTTOM_LEFT,
	horizontal: line::HORIZONTAL,
};

/// Displays a root key with child continuations as a tree.
#[derive(Debug, Clone, Default)]
pub struct KeyTree<'a> {
	/// The root key label (e.g., the pressed prefix).
	root: Cow<'a, str>,
	/// Child nodes representing available continuations.
	children: Vec<KeyTreeNode<'a>>,
	/// Symbols used for tree connectors.
	symbols: TreeSymbols<'a>,
	/// Style for the root key.
	root_style: Style,
	/// Style for child key labels.
	key_style: Style,
	/// Style for descriptions.
	desc_style: Style,
	/// Style for tree connector lines.
	line_style: Style,
}

impl<'a> KeyTree<'a> {
	/// Creates a new key tree with a root key and its continuations.
	pub fn new(root: impl Into<Cow<'a, str>>, children: Vec<KeyTreeNode<'a>>) -> Self {
		Self { root: root.into(), children, ..Default::default() }
	}

	/// Sets the tree line symbols.
	#[must_use]
	pub const fn symbols(mut self, symbols: TreeSymbols<'a>) -> Self {
		self.symbols = symbols;
		self
	}

	/// Sets the style for the root key.
	#[must_use]
	pub const fn root_style(mut self, style: Style) -> Self {
		self.root_style = style;
		self
	}

	/// Sets the style for child key labels.
	#[must_use]
	pub const fn key_style(mut self, style: Style) -> Self {
		self.key_style = style;
		self
	}

	/// Sets the style for descriptions.
	#[must_use]
	pub const fn desc_style(mut self, style: Style) -> Self {
		self.desc_style = style;
		self
	}

	/// Sets the style for tree connector lines.
	#[must_use]
	pub const fn line_style(mut self, style: Style) -> Self {
		self.line_style = style;
		self
	}
}

impl Widget for KeyTree<'_> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		if area.is_empty() || self.children.is_empty() {
			return;
		}

		let mut y = area.y;

		// Render root key
		let root_width = self.root.len().min(area.width as usize);
		buf.set_stringn(area.x, y, &self.root, root_width, self.root_style);
		y += 1;

		// Render children with tree connectors
		for (i, node) in self.children.iter().enumerate() {
			if y >= area.bottom() {
				break;
			}

			let is_last = i == self.children.len() - 1;
			let connector = if is_last { self.symbols.corner } else { self.symbols.branch };

			let mut x = area.x;
			buf.set_string(x, y, connector, self.line_style);
			x += 1;

			if x < area.right() {
				buf.set_string(x, y, self.symbols.horizontal, self.line_style);
				x += 1;
			}

			if x < area.right() {
				let key_width = node.key.len().min((area.right() - x) as usize);
				buf.set_stringn(x, y, &node.key, key_width, self.key_style);
				x += key_width as u16;
			}

			if x < area.right() {
				buf.set_string(x, y, " ", self.desc_style);
				x += 1;
			}

			if x < area.right() {
				let desc_width = node.description.len().min((area.right() - x) as usize);
				buf.set_stringn(x, y, &node.description, desc_width, self.desc_style);
			}

			y += 1;
		}
	}
}

#[cfg(test)]
mod tests {
	use alloc::string::{String, ToString};
	use alloc::vec;

	use super::*;

	fn render_to_lines(tree: KeyTree<'_>, width: u16, height: u16) -> Vec<String> {
		let area = Rect::new(0, 0, width, height);
		let mut buf = Buffer::empty(area);
		tree.render(area, &mut buf);

		(0..height)
			.map(|y| {
				(0..width)
					.map(|x| buf[(x, y)].symbol().to_string())
					.collect::<String>()
					.trim_end()
					.to_string()
			})
			.collect()
	}

	#[test]
	fn empty_tree_renders_nothing() {
		let tree = KeyTree::new("g", vec![]);
		let lines = render_to_lines(tree, 20, 5);
		assert!(lines.iter().all(|l| l.is_empty()));
	}

	#[test]
	fn root_with_single_child() {
		let children = vec![KeyTreeNode::new("g", "document_start")];
		let tree = KeyTree::new("g", children);
		let lines = render_to_lines(tree, 25, 3);
		assert_eq!(lines[0], "g");
		assert!(lines[1].contains("╰─g document_start"));
	}

	#[test]
	fn root_with_multiple_children() {
		let children = vec![
			KeyTreeNode::new("g", "start"),
			KeyTreeNode::new("e", "end"),
			KeyTreeNode::new("h", "home"),
		];
		let tree = KeyTree::new("g", children);
		let lines = render_to_lines(tree, 20, 5);

		assert_eq!(lines[0], "g");
		assert!(lines[1].contains("├─g"));
		assert!(lines[2].contains("├─e"));
		assert!(lines[3].contains("╰─h"));
	}

	#[test]
	fn truncates_to_area() {
		use unicode_width::UnicodeWidthStr;
		let children = vec![KeyTreeNode::new("g", "a very long description")];
		let tree = KeyTree::new("g", children);
		let lines = render_to_lines(tree, 12, 2);
		assert!(lines[1].width() <= 12);
	}
}
