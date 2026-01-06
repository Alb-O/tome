//! Popup anchor and positioning logic.

use xeno_tui::layout::Rect;

/// Size hints for popup dimensions.
#[derive(Debug, Clone, Copy, Default)]
pub struct SizeHints {
	/// Minimum width in characters.
	pub min_width: u16,
	/// Minimum height in lines.
	pub min_height: u16,
	/// Maximum width (0 = no limit).
	pub max_width: u16,
	/// Maximum height (0 = no limit).
	pub max_height: u16,
	/// Preferred width (0 = auto).
	pub preferred_width: u16,
	/// Preferred height (0 = auto).
	pub preferred_height: u16,
}

impl SizeHints {
	/// Creates size hints with only minimum dimensions specified.
	pub fn min(width: u16, height: u16) -> Self {
		Self {
			min_width: width,
			min_height: height,
			..Default::default()
		}
	}

	/// Creates size hints with both minimum and maximum dimensions.
	pub fn bounded(min_width: u16, min_height: u16, max_width: u16, max_height: u16) -> Self {
		Self {
			min_width,
			min_height,
			max_width,
			max_height,
			..Default::default()
		}
	}

	/// Creates size hints with preferred dimensions.
	pub fn preferred(width: u16, height: u16) -> Self {
		Self {
			preferred_width: width,
			preferred_height: height,
			..Default::default()
		}
	}

	/// Sets the maximum dimensions.
	pub fn with_max(mut self, max_width: u16, max_height: u16) -> Self {
		self.max_width = max_width;
		self.max_height = max_height;
		self
	}
}

/// Anchor point for popup positioning.
#[derive(Debug, Clone, Copy)]
pub enum PopupAnchor {
	/// Anchor to buffer cursor position.
	///
	/// The popup will be positioned relative to the cursor's screen coordinates.
	/// If `prefer_above` is true, the popup will appear above the cursor when possible.
	Cursor {
		/// Prefer positioning above the cursor.
		prefer_above: bool,
	},

	/// Anchor to a specific screen position.
	///
	/// The popup will be positioned at the specified screen coordinates.
	Position {
		/// Screen column (x coordinate).
		x: u16,
		/// Screen row (y coordinate).
		y: u16,
		/// Prefer positioning above the anchor point.
		prefer_above: bool,
	},

	/// Center the popup on screen.
	Center,
}

impl Default for PopupAnchor {
	fn default() -> Self {
		Self::Cursor {
			prefer_above: false,
		}
	}
}

impl PopupAnchor {
	/// Creates a cursor anchor that prefers positioning below the cursor.
	pub fn cursor_below() -> Self {
		Self::Cursor {
			prefer_above: false,
		}
	}

	/// Creates a cursor anchor that prefers positioning above the cursor.
	pub fn cursor_above() -> Self {
		Self::Cursor { prefer_above: true }
	}

	/// Creates a position anchor at the given screen coordinates.
	pub fn at(x: u16, y: u16) -> Self {
		Self::Position {
			x,
			y,
			prefer_above: false,
		}
	}

	/// Creates a centered anchor.
	pub fn center() -> Self {
		Self::Center
	}
}

/// Calculates the final position for a popup with collision detection.
///
/// This function determines where to place a popup given:
/// - The available screen area
/// - The anchor point (cursor position or fixed position)
/// - The popup's size hints
/// - The actual content size
///
/// # Arguments
///
/// * `screen` - The available screen area
/// * `anchor` - The anchor point for positioning
/// * `cursor_pos` - The cursor's screen position (x, y), if known
/// * `hints` - Size constraints for the popup
/// * `content_width` - The actual content width
/// * `content_height` - The actual content height
///
/// # Returns
///
/// The calculated [`Rect`] for the popup position and size.
pub fn calculate_popup_position(
	screen: Rect,
	anchor: PopupAnchor,
	cursor_pos: Option<(u16, u16)>,
	hints: SizeHints,
	content_width: u16,
	content_height: u16,
) -> Rect {
	// Calculate final dimensions respecting constraints
	let width = calculate_dimension(
		content_width,
		hints.min_width,
		hints.max_width,
		hints.preferred_width,
		screen.width.saturating_sub(2), // Leave margin
	);

	let height = calculate_dimension(
		content_height,
		hints.min_height,
		hints.max_height,
		hints.preferred_height,
		screen.height.saturating_sub(2), // Leave margin
	);

	match anchor {
		PopupAnchor::Cursor { prefer_above } => {
			if let Some((cx, cy)) = cursor_pos {
				position_near_point(screen, cx, cy, width, height, prefer_above)
			} else {
				// Fallback to center if no cursor position
				position_centered(screen, width, height)
			}
		}
		PopupAnchor::Position { x, y, prefer_above } => {
			position_near_point(screen, x, y, width, height, prefer_above)
		}
		PopupAnchor::Center => position_centered(screen, width, height),
	}
}

/// Calculates a dimension respecting min/max/preferred constraints.
fn calculate_dimension(content: u16, min: u16, max: u16, preferred: u16, available: u16) -> u16 {
	let base = if preferred > 0 { preferred } else { content };

	let with_min = base.max(min);
	let with_max = if max > 0 { with_min.min(max) } else { with_min };

	with_max.min(available)
}

/// Positions a popup near a point with collision detection.
fn position_near_point(
	screen: Rect,
	x: u16,
	y: u16,
	width: u16,
	height: u16,
	prefer_above: bool,
) -> Rect {
	// Calculate horizontal position (align left edge with point, adjust if overflows)
	let popup_x = if x + width <= screen.x + screen.width {
		x
	} else {
		// Shift left to fit
		(screen.x + screen.width).saturating_sub(width)
	};

	// Ensure we don't go past the left edge
	let popup_x = popup_x.max(screen.x);

	// Calculate vertical position with preference and collision detection
	let space_above = y.saturating_sub(screen.y);
	let space_below = (screen.y + screen.height).saturating_sub(y + 1);

	let popup_y = if prefer_above {
		// Try above first
		if space_above >= height {
			y.saturating_sub(height)
		} else if space_below >= height {
			y + 1
		} else {
			// Neither fits perfectly, use the larger space
			if space_above >= space_below {
				y.saturating_sub(height.min(space_above))
			} else {
				y + 1
			}
		}
	} else {
		// Try below first
		if space_below >= height {
			y + 1
		} else if space_above >= height {
			y.saturating_sub(height)
		} else {
			// Neither fits perfectly, use the larger space
			if space_below >= space_above {
				y + 1
			} else {
				y.saturating_sub(height.min(space_above))
			}
		}
	};

	// Clamp to screen bounds
	let popup_y = popup_y.max(screen.y);
	let final_height = height.min((screen.y + screen.height).saturating_sub(popup_y));
	let final_width = width.min((screen.x + screen.width).saturating_sub(popup_x));

	Rect::new(popup_x, popup_y, final_width, final_height)
}

/// Positions a popup centered on screen.
fn position_centered(screen: Rect, width: u16, height: u16) -> Rect {
	let x = screen.x + (screen.width.saturating_sub(width)) / 2;
	let y = screen.y + (screen.height.saturating_sub(height)) / 2;

	Rect::new(x, y, width, height)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_position_below_cursor() {
		let screen = Rect::new(0, 0, 80, 24);
		let hints = SizeHints::default();

		let pos = calculate_popup_position(
			screen,
			PopupAnchor::Cursor {
				prefer_above: false,
			},
			Some((10, 5)),
			hints,
			20,
			5,
		);

		// Should be positioned below cursor
		assert_eq!(pos.y, 6);
		assert_eq!(pos.x, 10);
		assert_eq!(pos.width, 20);
		assert_eq!(pos.height, 5);
	}

	#[test]
	fn test_position_above_cursor_when_preferred() {
		let screen = Rect::new(0, 0, 80, 24);
		let hints = SizeHints::default();

		let pos = calculate_popup_position(
			screen,
			PopupAnchor::Cursor { prefer_above: true },
			Some((10, 10)),
			hints,
			20,
			5,
		);

		// Should be positioned above cursor
		assert_eq!(pos.y, 5);
		assert_eq!(pos.x, 10);
	}

	#[test]
	fn test_collision_detection_right_edge() {
		let screen = Rect::new(0, 0, 80, 24);
		let hints = SizeHints::default();

		let pos = calculate_popup_position(
			screen,
			PopupAnchor::Cursor {
				prefer_above: false,
			},
			Some((70, 5)),
			hints,
			20,
			5,
		);

		// Should shift left to fit
		assert!(pos.x + pos.width <= 80);
	}

	#[test]
	fn test_centered_positioning() {
		let screen = Rect::new(0, 0, 80, 24);
		let hints = SizeHints::default();

		let pos = calculate_popup_position(screen, PopupAnchor::Center, None, hints, 20, 10);

		// Should be centered
		assert_eq!(pos.x, 30);
		assert_eq!(pos.y, 7);
	}
}
