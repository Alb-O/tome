//! Regex-based selection manipulation actions.

use tome_manifest::actions::{ActionMode, ActionResult};

use crate::action;

action!(
	select_regex,
	{ description: "Select regex matches within selection" },
	result: ActionResult::ModeChange(ActionMode::SelectRegex)
);
action!(
	split_regex,
	{ description: "Split selection on regex matches" },
	result: ActionResult::ModeChange(ActionMode::SplitRegex)
);
action!(
	split_lines,
	{ description: "Split selection into lines" },
	result: ActionResult::SplitLines
);
action!(
	keep_matching,
	{ description: "Keep selections matching regex" },
	result: ActionResult::ModeChange(ActionMode::KeepMatching)
);
action!(
	keep_not_matching,
	{ description: "Keep selections not matching regex" },
	result: ActionResult::ModeChange(ActionMode::KeepNotMatching)
);
