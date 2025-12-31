//! Menu item types.

use alloc::borrow::Cow;
use alloc::vec::Vec;

/// A node in the menu tree.
///
/// Items can be either leaf items (selectable) or groups (containers with children).
/// Groups display as expandable submenus when highlighted.
pub struct MenuItem<T> {
	pub(crate) name: Cow<'static, str>,
	pub(crate) data: Option<T>,
	pub(crate) children: Vec<MenuItem<T>>,
	pub(crate) highlighted: bool,
}

impl<T> MenuItem<T> {
	/// Creates a selectable menu item.
	pub fn item(name: impl Into<Cow<'static, str>>, data: T) -> Self {
		Self {
			name: name.into(),
			data: Some(data),
			children: Vec::new(),
			highlighted: false,
		}
	}

	/// Creates a group (submenu container).
	pub fn group(name: impl Into<Cow<'static, str>>, children: Vec<Self>) -> Self {
		Self {
			name: name.into(),
			data: None,
			children,
			highlighted: false,
		}
	}

	/// Returns true if this item has children (is a group).
	pub fn is_group(&self) -> bool {
		!self.children.is_empty()
	}

	/// Returns the item's display name.
	pub fn name(&self) -> &str {
		&self.name
	}

	pub(crate) fn highlight_first_child(&mut self) -> bool {
		if let Some(child) = self.children.first_mut() {
			child.highlighted = true;
			true
		} else {
			false
		}
	}

	pub(crate) fn highlighted_child_index(&self) -> Option<usize> {
		self.children.iter().position(|c| c.highlighted)
	}

	pub(crate) fn highlighted_child(&self) -> Option<&Self> {
		self.children.iter().find(|c| c.highlighted)
	}

	pub(crate) fn highlighted_child_mut(&mut self) -> Option<&mut Self> {
		self.children.iter_mut().find(|c| c.highlighted)
	}

	pub(crate) fn clear_highlight(&mut self) {
		self.highlighted = false;
		for child in &mut self.children {
			child.clear_highlight();
		}
	}

	pub(crate) fn highlight_prev(&mut self) {
		let Some(idx) = self.highlighted_child_index() else {
			self.highlight_first_child();
			return;
		};
		let new_idx = idx.saturating_sub(1);
		self.children[idx].clear_highlight();
		self.children[new_idx].highlighted = true;
	}

	pub(crate) fn highlight_next(&mut self) {
		let Some(idx) = self.highlighted_child_index() else {
			self.highlight_first_child();
			return;
		};
		let new_idx = (idx + 1).min(self.children.len().saturating_sub(1));
		self.children[idx].clear_highlight();
		self.children[new_idx].highlighted = true;
	}

	/// Returns the deepest highlighted item.
	pub fn highlight(&self) -> Option<&Self> {
		if !self.highlighted {
			return None;
		}
		let mut current = self;
		while let Some(child) = current.highlighted_child() {
			current = child;
		}
		Some(current)
	}

	pub(crate) fn highlight_mut(&mut self) -> Option<&mut Self> {
		if !self.highlighted {
			return None;
		}
		let mut current = self;
		while current.highlighted_child().is_some() {
			current = current.highlighted_child_mut().unwrap();
		}
		Some(current)
	}

	/// Returns the second-to-last highlighted item (parent of deepest).
	pub(crate) fn highlight_parent_mut(&mut self) -> Option<&mut Self> {
		if !self.highlighted || self.highlighted_child().is_none() {
			return None;
		}
		let mut current = self;
		while current
			.highlighted_child()
			.and_then(|c| c.highlighted_child())
			.is_some()
		{
			current = current.highlighted_child_mut().unwrap();
		}
		Some(current)
	}
}
