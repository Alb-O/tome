//! LSP-related actions.

use crate::{ActionResult, action};

action!(
	show_hover,
	{
		description: "Show hover information at cursor",
		short_desc: "Hover info",
		bindings: r#"normal "K""#,
	},
	|_ctx| ActionResult::Command {
		name: "hover",
		args: vec![],
	}
);

action!(
	trigger_completion,
	{
		description: "Trigger completion menu",
		short_desc: "Completion",
		bindings: r#"insert "ctrl-space""#,
	},
	|_ctx| ActionResult::Command {
		name: "complete",
		args: vec![],
	}
);

action!(
	goto_definition,
	{
		description: "Go to definition",
		short_desc: "Go to def",
		bindings: r#"normal "g d""#,
	},
	|_ctx| ActionResult::Command {
		name: "definition",
		args: vec![],
	}
);

action!(
	find_references,
	{
		description: "Find references",
		short_desc: "References",
		bindings: r#"normal "g r""#,
	},
	|_ctx| ActionResult::Command {
		name: "references",
		args: vec![],
	}
);

action!(
	show_code_actions,
	{
		description: "Show code actions at cursor",
		short_desc: "Code actions",
		bindings: r#"space "a""#,
	},
	|_ctx| ActionResult::Command {
		name: "code_actions",
		args: vec![],
	}
);
