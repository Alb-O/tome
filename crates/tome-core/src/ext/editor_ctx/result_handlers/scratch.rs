//! Scratch buffer result handlers.

use linkme::distributed_slice;

use crate::ext::actions::ActionResult;
use crate::ext::editor_ctx::{HandleOutcome, ResultHandler, RESULT_HANDLERS};

#[distributed_slice(RESULT_HANDLERS)]
static HANDLE_OPEN_SCRATCH: ResultHandler = ResultHandler {
    name: "open_scratch",
    handles: |r| matches!(r, ActionResult::OpenScratch { .. }),
    handle: |r, ctx, _| {
        if let ActionResult::OpenScratch { focus } = r
            && let Some(scratch) = ctx.scratch() {
                scratch.open(*focus);
            }
        HandleOutcome::Handled
    },
};

#[distributed_slice(RESULT_HANDLERS)]
static HANDLE_CLOSE_SCRATCH: ResultHandler = ResultHandler {
    name: "close_scratch",
    handles: |r| matches!(r, ActionResult::CloseScratch),
    handle: |_, ctx, _| {
        if let Some(scratch) = ctx.scratch() {
            scratch.close();
        }
        HandleOutcome::Handled
    },
};

#[distributed_slice(RESULT_HANDLERS)]
static HANDLE_TOGGLE_SCRATCH: ResultHandler = ResultHandler {
    name: "toggle_scratch",
    handles: |r| matches!(r, ActionResult::ToggleScratch),
    handle: |_, ctx, _| {
        if let Some(scratch) = ctx.scratch() {
            scratch.toggle();
        }
        HandleOutcome::Handled
    },
};

#[distributed_slice(RESULT_HANDLERS)]
static HANDLE_EXECUTE_SCRATCH: ResultHandler = ResultHandler {
    name: "execute_scratch",
    handles: |r| matches!(r, ActionResult::ExecuteScratch),
    handle: |_, ctx, _| {
        if let Some(scratch) = ctx.scratch()
            && scratch.execute() {
                return HandleOutcome::Quit;
            }
        HandleOutcome::Handled
    },
};
