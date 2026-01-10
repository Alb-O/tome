//! Sign column for diagnostics, breakpoints, and custom markers.

use crate::{GutterCell, GutterStyle, gutter};

gutter!(signs, {
	description: "Sign column for diagnostics and markers",
	priority: -10,
	width: Fixed(2),
	enabled: true
}, |ctx| {
	// Custom signs take priority (breakpoints, bookmarks, etc.)
	if let Some(sign) = ctx.annotations.sign {
		return Some(GutterCell {
			text: sign.to_string(),
			style: GutterStyle::Normal,
		});
	}

	// Diagnostic signs with semantic styling
	match ctx.annotations.diagnostic_severity {
		4 => Some(GutterCell { text: "●".into(), style: GutterStyle::Error }),
		3 => Some(GutterCell { text: "●".into(), style: GutterStyle::Warning }),
		2 => Some(GutterCell { text: "●".into(), style: GutterStyle::Info }),
		1 => Some(GutterCell { text: "●".into(), style: GutterStyle::Hint }),
		_ => None,
	}
});
