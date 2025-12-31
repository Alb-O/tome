//! Menu layout regions for hit-testing.

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::layout::Rect;

/// Computed regions for menu hit-testing.
pub struct MenuLayout {
	/// Hit regions for each bar item.
	pub bar_regions: Vec<Rect>,
	/// Layout for the active dropdown, if any.
	pub dropdown: Option<DropdownLayout>,
}

/// Layout information for a dropdown panel (recursive for nested submenus).
pub struct DropdownLayout {
	/// Total dropdown area including borders.
	pub area: Rect,
	/// Hit regions for each dropdown item.
	pub item_regions: Vec<Rect>,
	/// Nested submenu layout, if any.
	pub submenu: Option<Box<DropdownLayout>>,
}
