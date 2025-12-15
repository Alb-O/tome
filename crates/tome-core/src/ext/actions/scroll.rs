//! Scroll/view actions.

use linkme::distributed_slice;

use crate::ext::actions::{ActionDef, ActionResult, ACTIONS};

#[distributed_slice(ACTIONS)]
static ACTION_SCROLL_UP: ActionDef = ActionDef {
    name: "scroll_up",
    description: "Scroll view up",
    handler: |_ctx| ActionResult::Ok,
};

#[distributed_slice(ACTIONS)]
static ACTION_SCROLL_DOWN: ActionDef = ActionDef {
    name: "scroll_down",
    description: "Scroll view down",
    handler: |_ctx| ActionResult::Ok,
};

#[distributed_slice(ACTIONS)]
static ACTION_SCROLL_HALF_PAGE_UP: ActionDef = ActionDef {
    name: "scroll_half_page_up",
    description: "Scroll half page up",
    handler: |_ctx| ActionResult::Ok,
};

#[distributed_slice(ACTIONS)]
static ACTION_SCROLL_HALF_PAGE_DOWN: ActionDef = ActionDef {
    name: "scroll_half_page_down",
    description: "Scroll half page down",
    handler: |_ctx| ActionResult::Ok,
};

#[distributed_slice(ACTIONS)]
static ACTION_SCROLL_PAGE_UP: ActionDef = ActionDef {
    name: "scroll_page_up",
    description: "Scroll page up",
    handler: |_ctx| ActionResult::Ok,
};

#[distributed_slice(ACTIONS)]
static ACTION_SCROLL_PAGE_DOWN: ActionDef = ActionDef {
    name: "scroll_page_down",
    description: "Scroll page down",
    handler: |_ctx| ActionResult::Ok,
};

#[distributed_slice(ACTIONS)]
static ACTION_CENTER_CURSOR: ActionDef = ActionDef {
    name: "center_cursor",
    description: "Center cursor in view",
    handler: |_ctx| ActionResult::Ok,
};

#[distributed_slice(ACTIONS)]
static ACTION_CURSOR_TO_TOP: ActionDef = ActionDef {
    name: "cursor_to_top",
    description: "Move view so cursor is at top",
    handler: |_ctx| ActionResult::Ok,
};

#[distributed_slice(ACTIONS)]
static ACTION_CURSOR_TO_BOTTOM: ActionDef = ActionDef {
    name: "cursor_to_bottom",
    description: "Move view so cursor is at bottom",
    handler: |_ctx| ActionResult::Ok,
};
