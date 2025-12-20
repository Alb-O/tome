use ratatui::layout::Rect;
use ratatui::widgets::block::Padding;

use crate::notifications::functions::fnc_get_level_icon::get_level_icon;
use crate::notifications::types::Level;

/// Terminal-cell width reserved for the icon glyph.
///
/// Nerd Font icons often render as 2 cells even when Unicode width APIs report 1.
pub const ICON_CELL_WIDTH: u16 = 2;

/// Left padding before the icon.
pub const GUTTER_LEFT_PAD: u16 = 0;

/// One cell gap between icon and content.
pub const GUTTER_RIGHT_PAD: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GutterLayout {
	pub left_pad: u16,
	pub icon_width: u16,
	pub right_pad: u16,
}

impl Default for GutterLayout {
	fn default() -> Self {
		Self {
			left_pad: GUTTER_LEFT_PAD,
			icon_width: ICON_CELL_WIDTH,
			right_pad: GUTTER_RIGHT_PAD,
		}
	}
}

impl GutterLayout {
	pub fn total_width(self) -> u16 {
		self.left_pad + self.icon_width + self.right_pad
	}
}

/// Returns the default gutter layout for a given level.
///
/// If no icon is available for the level, returns None.
pub fn gutter_layout(level: Option<Level>) -> Option<GutterLayout> {
	get_level_icon(level).map(|_| GutterLayout::default())
}

/// Applies gutter width as additional left padding for measurement.
///
/// This lets us measure wrapping/height with the same content width that
/// rendering uses (content is narrower by `gutter.total_width()`).
pub fn padding_with_gutter(padding: Padding, gutter: Option<GutterLayout>) -> Padding {
	match gutter {
		Some(g) => Padding {
			left: padding.left.saturating_add(g.total_width()),
			..padding
		},
		None => padding,
	}
}

/// Splits an inner notification area into `(gutter, content)` rects.
pub fn split_inner(inner: Rect, gutter: GutterLayout) -> (Rect, Rect) {
	let gutter_width = gutter.total_width().min(inner.width);
	let gutter_rect = Rect {
		x: inner.x,
		y: inner.y,
		width: gutter_width,
		height: inner.height,
	};
	let content_rect = Rect {
		x: inner.x.saturating_add(gutter_width),
		y: inner.y,
		width: inner.width.saturating_sub(gutter_width),
		height: inner.height,
	};
	(gutter_rect, content_rect)
}
