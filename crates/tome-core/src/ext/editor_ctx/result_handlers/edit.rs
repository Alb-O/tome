//! Edit action result handler.
//!
//! Edit operations (delete, yank, paste, etc.) require more complex editor
//! access. These are handled via the EditAccess capability trait.

use linkme::distributed_slice;

use crate::ext::actions::ActionResult;
use crate::ext::editor_ctx::{HandleOutcome, ResultHandler, RESULT_HANDLERS};

#[distributed_slice(RESULT_HANDLERS)]
static HANDLE_EDIT: ResultHandler = ResultHandler {
    name: "edit",
    handles: |r| matches!(r, ActionResult::Edit(_)),
    handle: |r, ctx, extend| {
        if let ActionResult::Edit(action) = r {
            // Edit operations need to be handled by the terminal layer
            // since they require transaction support, undo stack, etc.
            // For now, signal that we didn't handle it so tome-term can.
            // 
            // TODO: Add EditAccess capability trait and implement handlers
            // for each EditAction variant.
            let _ = (action, ctx, extend);
            return HandleOutcome::NotHandled;
        }
        HandleOutcome::Handled
    },
};
