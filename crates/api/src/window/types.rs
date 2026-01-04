//! Window and floating window types.

use xeno_tui::layout::Rect;

use crate::buffer::{BufferId, Layout};

/// Unique identifier for a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId(pub(crate) u64);

/// Window kinds.
pub enum Window {
	/// The base window containing the split tree.
	Base(BaseWindow),
	/// A floating window positioned over content.
	Floating(FloatingWindow),
}

/// The main editor window with split layout.
pub struct BaseWindow {
	pub layout: Layout,
	pub focused_buffer: BufferId,
}

/// A floating window with absolute positioning.
#[derive(Debug, Clone)]
pub struct FloatingWindow {
	pub id: WindowId,
	pub buffer: BufferId,
	pub rect: Rect,
	/// If true, resists losing focus from mouse hover.
	pub sticky: bool,
	/// If true, closes when focus is lost.
	pub dismiss_on_blur: bool,
	/// Visual style (border, shadow, transparency).
	pub style: FloatingStyle,
}

/// Visual style for floating windows.
#[derive(Debug, Clone)]
pub struct FloatingStyle {
	pub border: bool,
	pub shadow: bool,
	pub title: Option<String>,
}
