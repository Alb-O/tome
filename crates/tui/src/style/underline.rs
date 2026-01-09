//! Underline style types for terminal rendering.

use core::fmt;
use core::str::FromStr;

/// The style of underline to use for text.
///
/// Modern terminals like Kitty, iTerm2, and WezTerm support extended underline styles
/// beyond the basic single line. This enum allows specifying the desired style.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum UnderlineStyle {
	/// No underline / reset underline.
	#[default]
	Reset,
	/// Standard single line underline.
	Line,
	/// Curly/wavy underline (commonly used for spell-check and diagnostics).
	Curl,
	/// Dotted underline.
	Dotted,
	/// Dashed underline.
	Dashed,
	/// Double line underline.
	DoubleLine,
}

impl fmt::Display for UnderlineStyle {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Reset => write!(f, "reset"),
			Self::Line => write!(f, "line"),
			Self::Curl => write!(f, "curl"),
			Self::Dotted => write!(f, "dotted"),
			Self::Dashed => write!(f, "dashed"),
			Self::DoubleLine => write!(f, "double_line"),
		}
	}
}

impl FromStr for UnderlineStyle {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"reset" | "none" => Ok(Self::Reset),
			"line" | "single" => Ok(Self::Line),
			"curl" | "curly" => Ok(Self::Curl),
			"dotted" => Ok(Self::Dotted),
			"dashed" => Ok(Self::Dashed),
			"double_line" | "double" => Ok(Self::DoubleLine),
			_ => Err("invalid underline style"),
		}
	}
}
