//! Theme option.

use evildoer_manifest::theme::DEFAULT_THEME_ID;

use crate::option;

option!(
	theme,
	String,
	DEFAULT_THEME_ID.to_string(),
	Global,
	"Editor color theme"
);
