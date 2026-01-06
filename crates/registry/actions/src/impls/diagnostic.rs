//! Diagnostic navigation actions.

use crate::{ActionResult, action};

action!(
	goto_next_diagnostic,
	{
		description: "Go to next diagnostic",
		short_desc: "Next diagnostic",
		bindings: r#"normal "] d""#,
	},
	|_ctx| ActionResult::Command {
		name: "diagnostic_next",
		args: vec![],
	}
);

action!(
	goto_prev_diagnostic,
	{
		description: "Go to previous diagnostic",
		short_desc: "Prev diagnostic",
		bindings: r#"normal "[ d""#,
	},
	|_ctx| ActionResult::Command {
		name: "diagnostic_prev",
		args: vec![],
	}
);
