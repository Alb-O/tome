//! Core result handlers: Ok, CursorMove, Motion, Edit, Quit, Error.

use linkme::distributed_slice;

use crate::ext::actions::ActionResult;
use crate::ext::editor_ctx::{HandleOutcome, ResultHandler, RESULT_HANDLERS};
use crate::Mode;

#[distributed_slice(RESULT_HANDLERS)]
static HANDLE_OK: ResultHandler = ResultHandler {
    name: "ok",
    handles: |r| matches!(r, ActionResult::Ok),
    handle: |_, _, _| HandleOutcome::Handled,
};

#[distributed_slice(RESULT_HANDLERS)]
static HANDLE_CURSOR_MOVE: ResultHandler = ResultHandler {
    name: "cursor_move",
    handles: |r| matches!(r, ActionResult::CursorMove(_)),
    handle: |r, ctx, _| {
        if let ActionResult::CursorMove(pos) = r {
            ctx.set_cursor(*pos);
        }
        HandleOutcome::Handled
    },
};

#[distributed_slice(RESULT_HANDLERS)]
static HANDLE_MOTION: ResultHandler = ResultHandler {
    name: "motion",
    handles: |r| matches!(r, ActionResult::Motion(_)),
    handle: |r, ctx, _| {
        if let ActionResult::Motion(sel) = r {
            ctx.set_cursor(sel.primary().head);
            ctx.set_selection(sel.clone());
        }
        HandleOutcome::Handled
    },
};

#[distributed_slice(RESULT_HANDLERS)]
static HANDLE_INSERT_WITH_MOTION: ResultHandler = ResultHandler {
    name: "insert_with_motion",
    handles: |r| matches!(r, ActionResult::InsertWithMotion(_)),
    handle: |r, ctx, _| {
        if let ActionResult::InsertWithMotion(sel) = r {
            ctx.set_cursor(sel.primary().head);
            ctx.set_selection(sel.clone());
            ctx.set_mode(Mode::Insert);
        }
        HandleOutcome::Handled
    },
};

#[distributed_slice(RESULT_HANDLERS)]
static HANDLE_QUIT: ResultHandler = ResultHandler {
    name: "quit",
    handles: |r| matches!(r, ActionResult::Quit | ActionResult::ForceQuit),
    handle: |_, _, _| HandleOutcome::Quit,
};

#[distributed_slice(RESULT_HANDLERS)]
static HANDLE_ERROR: ResultHandler = ResultHandler {
    name: "error",
    handles: |r| matches!(r, ActionResult::Error(_)),
    handle: |r, ctx, _| {
        if let ActionResult::Error(msg) = r {
            ctx.message(msg);
        }
        HandleOutcome::Handled
    },
};

#[distributed_slice(RESULT_HANDLERS)]
static HANDLE_PENDING: ResultHandler = ResultHandler {
    name: "pending",
    handles: |r| matches!(r, ActionResult::Pending(_)),
    handle: |r, ctx, _| {
        if let ActionResult::Pending(pending) = r {
            ctx.message(&pending.prompt);
            ctx.set_mode(Mode::PendingAction(pending.kind));
        }
        HandleOutcome::Handled
    },
};

#[distributed_slice(RESULT_HANDLERS)]
static HANDLE_FORCE_REDRAW: ResultHandler = ResultHandler {
    name: "force_redraw",
    handles: |r| matches!(r, ActionResult::ForceRedraw),
    handle: |_, _, _| HandleOutcome::Handled,
};
