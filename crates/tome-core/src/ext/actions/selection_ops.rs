//! Selection manipulation actions (collapse, flip, select all, etc.).

use linkme::distributed_slice;

use crate::ext::actions::{ActionContext, ActionDef, ActionResult, ACTIONS};
use crate::selection::Selection;

fn collapse_selection(ctx: &ActionContext) -> ActionResult {
    let mut new_sel = ctx.selection.clone();
    new_sel.transform_mut(|r| {
        r.anchor = r.head;
    });
    ActionResult::Motion(new_sel)
}

#[distributed_slice(ACTIONS)]
static ACTION_COLLAPSE_SELECTION: ActionDef = ActionDef {
    name: "collapse_selection",
    description: "Collapse selection to cursor",
    handler: collapse_selection,
};

fn flip_selection(ctx: &ActionContext) -> ActionResult {
    let mut new_sel = ctx.selection.clone();
    new_sel.transform_mut(|r| {
        std::mem::swap(&mut r.anchor, &mut r.head);
    });
    ActionResult::Motion(new_sel)
}

#[distributed_slice(ACTIONS)]
static ACTION_FLIP_SELECTION: ActionDef = ActionDef {
    name: "flip_selection",
    description: "Flip selection direction",
    handler: flip_selection,
};

fn ensure_forward(ctx: &ActionContext) -> ActionResult {
    let mut new_sel = ctx.selection.clone();
    new_sel.transform_mut(|r| {
        if r.head < r.anchor {
            std::mem::swap(&mut r.anchor, &mut r.head);
        }
    });
    ActionResult::Motion(new_sel)
}

#[distributed_slice(ACTIONS)]
static ACTION_ENSURE_FORWARD: ActionDef = ActionDef {
    name: "ensure_forward",
    description: "Ensure selection is forward",
    handler: ensure_forward,
};

fn select_line(ctx: &ActionContext) -> ActionResult {
    let mut new_sel = ctx.selection.clone();
    new_sel.transform_mut(|r| {
        let line = ctx.text.char_to_line(r.head);
        let start = ctx.text.line_to_char(line);
        let end = if line + 1 < ctx.text.len_lines() {
            ctx.text.line_to_char(line + 1)
        } else {
            ctx.text.len_chars()
        };
        r.anchor = start;
        r.head = end;
    });
    ActionResult::Motion(new_sel)
}

#[distributed_slice(ACTIONS)]
static ACTION_SELECT_LINE: ActionDef = ActionDef {
    name: "select_line",
    description: "Select whole line",
    handler: select_line,
};

fn select_all(ctx: &ActionContext) -> ActionResult {
    ActionResult::Motion(Selection::single(0, ctx.text.len_chars()))
}

#[distributed_slice(ACTIONS)]
static ACTION_SELECT_ALL: ActionDef = ActionDef {
    name: "select_all",
    description: "Select entire buffer",
    handler: select_all,
};

fn keep_primary_selection(ctx: &ActionContext) -> ActionResult {
    let primary = ctx.selection.primary();
    ActionResult::Motion(Selection::single(primary.anchor, primary.head))
}

#[distributed_slice(ACTIONS)]
static ACTION_KEEP_PRIMARY: ActionDef = ActionDef {
    name: "keep_primary_selection",
    description: "Keep only primary selection",
    handler: keep_primary_selection,
};

fn rotate_selections_forward(ctx: &ActionContext) -> ActionResult {
    let mut new_sel = ctx.selection.clone();
    new_sel.rotate_forward();
    ActionResult::Motion(new_sel)
}

#[distributed_slice(ACTIONS)]
static ACTION_ROTATE_FORWARD: ActionDef = ActionDef {
    name: "rotate_selections_forward",
    description: "Rotate selections forward",
    handler: rotate_selections_forward,
};

fn rotate_selections_backward(ctx: &ActionContext) -> ActionResult {
    let mut new_sel = ctx.selection.clone();
    new_sel.rotate_backward();
    ActionResult::Motion(new_sel)
}

#[distributed_slice(ACTIONS)]
static ACTION_ROTATE_BACKWARD: ActionDef = ActionDef {
    name: "rotate_selections_backward",
    description: "Rotate selections backward",
    handler: rotate_selections_backward,
};

fn escape(ctx: &ActionContext) -> ActionResult {
    let mut new_sel = ctx.selection.clone();
    new_sel.transform_mut(|r| {
        r.anchor = r.head;
    });
    ActionResult::Motion(new_sel)
}

#[distributed_slice(ACTIONS)]
static ACTION_ESCAPE: ActionDef = ActionDef {
    name: "escape",
    description: "Escape (collapse selection)",
    handler: escape,
};

fn remove_primary_selection(ctx: &ActionContext) -> ActionResult {
    if ctx.selection.ranges().len() <= 1 {
        return ActionResult::Ok;
    }
    let mut new_sel = ctx.selection.clone();
    new_sel.remove_primary();
    ActionResult::Motion(new_sel)
}

#[distributed_slice(ACTIONS)]
static ACTION_REMOVE_PRIMARY: ActionDef = ActionDef {
    name: "remove_primary_selection",
    description: "Remove primary selection",
    handler: remove_primary_selection,
};

fn trim_to_line(ctx: &ActionContext) -> ActionResult {
    let mut new_sel = ctx.selection.clone();
    new_sel.transform_mut(|r| {
        let line = ctx.text.char_to_line(r.head);
        let start = ctx.text.line_to_char(line);
        let end = if line + 1 < ctx.text.len_lines() {
            ctx.text.line_to_char(line + 1).saturating_sub(1)
        } else {
            ctx.text.len_chars()
        };
        r.anchor = r.anchor.max(start).min(end);
        r.head = r.head.max(start).min(end);
    });
    ActionResult::Motion(new_sel)
}

#[distributed_slice(ACTIONS)]
static ACTION_TRIM_TO_LINE: ActionDef = ActionDef {
    name: "trim_to_line",
    description: "Trim selection to line boundaries",
    handler: trim_to_line,
};
