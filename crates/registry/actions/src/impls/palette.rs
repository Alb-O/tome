//! Command palette actions.

use crate::{ActionResult, action};

action!(open_palette, {
	description: "Open command palette",
	bindings: r#"normal ":""#,
}, |_ctx| ActionResult::OpenPalette);
