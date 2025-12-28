//! Result handlers for selection-oriented operations.

use tome_manifest::editor_ctx::HandleOutcome;
use tome_manifest::result_handler;

use crate::NotifyWARNExt;

result_handler!(
	RESULT_SPLIT_LINES_HANDLERS,
	HANDLE_SPLIT_LINES,
	"split_lines",
	|_, ctx, _| {
		if let Some(ops) = ctx.selection_ops() {
			ops.split_lines();
			HandleOutcome::Handled
		} else {
			ctx.warn("Split lines not available");
			HandleOutcome::Handled
		}
	}
);
