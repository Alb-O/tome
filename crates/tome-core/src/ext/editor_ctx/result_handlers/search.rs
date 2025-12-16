//! Search result handlers.

use linkme::distributed_slice;

use crate::ext::actions::ActionResult;
use crate::ext::editor_ctx::{HandleOutcome, ResultHandler, RESULT_HANDLERS};

#[distributed_slice(RESULT_HANDLERS)]
static HANDLE_SEARCH_NEXT: ResultHandler = ResultHandler {
    name: "search_next",
    handles: |r| matches!(r, ActionResult::SearchNext { .. }),
    handle: |r, ctx, extend| {
        if let ActionResult::SearchNext { add_selection } = r
            && let Some(search) = ctx.search() {
                search.search_next(*add_selection, extend);
            }
        HandleOutcome::Handled
    },
};

#[distributed_slice(RESULT_HANDLERS)]
static HANDLE_SEARCH_PREV: ResultHandler = ResultHandler {
    name: "search_prev",
    handles: |r| matches!(r, ActionResult::SearchPrev { .. }),
    handle: |r, ctx, extend| {
        if let ActionResult::SearchPrev { add_selection } = r
            && let Some(search) = ctx.search() {
                search.search_prev(*add_selection, extend);
            }
        HandleOutcome::Handled
    },
};

#[distributed_slice(RESULT_HANDLERS)]
static HANDLE_USE_SELECTION_AS_SEARCH: ResultHandler = ResultHandler {
    name: "use_selection_as_search",
    handles: |r| matches!(r, ActionResult::UseSelectionAsSearch),
    handle: |_, ctx, _| {
        if let Some(search) = ctx.search() {
            search.use_selection_as_pattern();
        }
        HandleOutcome::Handled
    },
};
