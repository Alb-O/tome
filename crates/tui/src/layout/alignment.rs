use strum::{Display, EnumString};

/// Horizontal content alignment within a layout area.
///
/// This type is used throughout Ratatui to control how content is positioned horizontally within
/// available space. It's commonly used with widgets to control text alignment, but can also be
/// used in layout calculations.
///
/// For comprehensive layout documentation and examples, see the [`layout`](crate::layout) module.
#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HorizontalAlignment {
	/// Content is aligned to the left side of the area.
	#[default]
	Left,
	/// Content is centered within the area.
	Center,
	/// Content is aligned to the right side of the area.
	Right,
}

/// Vertical content alignment within a layout area.
///
/// This type is used to control how content is positioned vertically within available space.
/// It complements [`HorizontalAlignment`] to provide full 2D positioning control.
///
/// For comprehensive layout documentation and examples, see the [`layout`](crate::layout) module.
#[derive(Debug, Default, Display, EnumString, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VerticalAlignment {
	/// Content is aligned to the top of the area.
	#[default]
	Top,
	/// Content is centered vertically within the area.
	Center,
	/// Content is aligned to the bottom of the area.
	Bottom,
}

#[cfg(test)]
mod tests {
	use alloc::string::ToString;

	use strum::ParseError;

	use super::*;

	#[test]
	fn horizontal_alignment_to_string() {
		assert_eq!(HorizontalAlignment::Left.to_string(), "Left");
		assert_eq!(HorizontalAlignment::Center.to_string(), "Center");
		assert_eq!(HorizontalAlignment::Right.to_string(), "Right");
	}

	#[test]
	fn horizontal_alignment_from_str() {
		assert_eq!(
			"Left".parse::<HorizontalAlignment>(),
			Ok(HorizontalAlignment::Left)
		);
		assert_eq!(
			"Center".parse::<HorizontalAlignment>(),
			Ok(HorizontalAlignment::Center)
		);
		assert_eq!(
			"Right".parse::<HorizontalAlignment>(),
			Ok(HorizontalAlignment::Right)
		);
		assert_eq!(
			"".parse::<HorizontalAlignment>(),
			Err(ParseError::VariantNotFound)
		);
	}

	#[test]
	fn vertical_alignment_to_string() {
		assert_eq!(VerticalAlignment::Top.to_string(), "Top");
		assert_eq!(VerticalAlignment::Center.to_string(), "Center");
		assert_eq!(VerticalAlignment::Bottom.to_string(), "Bottom");
	}

	#[test]
	fn vertical_alignment_from_str() {
		let top = "Top".parse::<VerticalAlignment>();
		assert_eq!(top, Ok(VerticalAlignment::Top));

		let center = "Center".parse::<VerticalAlignment>();
		assert_eq!(center, Ok(VerticalAlignment::Center));

		let bottom = "Bottom".parse::<VerticalAlignment>();
		assert_eq!(bottom, Ok(VerticalAlignment::Bottom));

		let invalid = "".parse::<VerticalAlignment>();
		assert_eq!(invalid, Err(ParseError::VariantNotFound));
	}
}
