//! Edit action result handler.

use evildoer_registry::{result_handler, ActionResult, HandleOutcome};

result_handler!(
	RESULT_EDIT_HANDLERS,
	HANDLE_EDIT,
	"edit",
	|r, ctx, extend| {
		if let ActionResult::Edit(action) = r
			&& let Some(edit) = ctx.edit()
		{
			edit.execute_edit(action, extend);
			return HandleOutcome::Handled;
		}
		HandleOutcome::NotHandled
	}
);
