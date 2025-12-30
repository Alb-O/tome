//! Window mode actions with colocated keybindings and handlers.
//!
//! Split names follow Vim/Helix conventions based on the divider line orientation:
//! - `split_horizontal` (Ctrl+w s): horizontal divider → windows stacked top/bottom
//! - `split_vertical` (Ctrl+w v): vertical divider → windows side-by-side left/right

use evildoer_manifest::{action, result_handler};

action!(split_horizontal, {
	description: "Split horizontally (new buffer below)",
	bindings: r#"window "s""#,
}, |_ctx| evildoer_manifest::actions::ActionResult::SplitHorizontal);

result_handler!(
	RESULT_SPLIT_HORIZONTAL_HANDLERS,
	HANDLE_SPLIT_HORIZONTAL,
	"split_horizontal",
	|r, ctx, _| {
		use evildoer_manifest::actions::ActionResult;
		use evildoer_manifest::editor_ctx::{HandleOutcome, MessageAccess};
		if matches!(r, ActionResult::SplitHorizontal) {
			if let Some(ops) = ctx.buffer_ops() {
				ops.split_horizontal();
			} else {
				ctx.notify("warning", "Buffer operations not available");
			}
		}
		HandleOutcome::Handled
	}
);

action!(split_vertical, {
	description: "Split vertically (new buffer to right)",
	bindings: r#"window "v""#,
}, |_ctx| evildoer_manifest::actions::ActionResult::SplitVertical);

result_handler!(
	RESULT_SPLIT_VERTICAL_HANDLERS,
	HANDLE_SPLIT_VERTICAL,
	"split_vertical",
	|r, ctx, _| {
		use evildoer_manifest::actions::ActionResult;
		use evildoer_manifest::editor_ctx::{HandleOutcome, MessageAccess};
		if matches!(r, ActionResult::SplitVertical) {
			if let Some(ops) = ctx.buffer_ops() {
				ops.split_vertical();
			} else {
				ctx.notify("warning", "Buffer operations not available");
			}
		}
		HandleOutcome::Handled
	}
);

action!(split_terminal_horizontal, {
	description: "Open terminal in horizontal split (below)",
	bindings: r#"window "t""#,
}, |_ctx| evildoer_manifest::actions::ActionResult::SplitTerminalHorizontal);

result_handler!(
	RESULT_SPLIT_TERMINAL_HORIZONTAL_HANDLERS,
	HANDLE_SPLIT_TERMINAL_HORIZONTAL,
	"split_terminal_horizontal",
	|r, ctx, _| {
		use evildoer_manifest::actions::ActionResult;
		use evildoer_manifest::editor_ctx::{HandleOutcome, MessageAccess};
		if matches!(r, ActionResult::SplitTerminalHorizontal) {
			if let Some(ops) = ctx.buffer_ops() {
				ops.split_terminal_horizontal();
			} else {
				ctx.notify("warning", "Buffer operations not available");
			}
		}
		HandleOutcome::Handled
	}
);

action!(split_terminal_vertical, {
	description: "Open terminal in vertical split (right)",
	bindings: r#"window "T""#,
}, |_ctx| evildoer_manifest::actions::ActionResult::SplitTerminalVertical);

result_handler!(
	RESULT_SPLIT_TERMINAL_VERTICAL_HANDLERS,
	HANDLE_SPLIT_TERMINAL_VERTICAL,
	"split_terminal_vertical",
	|r, ctx, _| {
		use evildoer_manifest::actions::ActionResult;
		use evildoer_manifest::editor_ctx::{HandleOutcome, MessageAccess};
		if matches!(r, ActionResult::SplitTerminalVertical) {
			if let Some(ops) = ctx.buffer_ops() {
				ops.split_terminal_vertical();
			} else {
				ctx.notify("warning", "Buffer operations not available");
			}
		}
		HandleOutcome::Handled
	}
);

action!(toggle_terminal, {
	description: "Toggle terminal split",
	bindings: r#"normal ":""#,
}, |_ctx| evildoer_manifest::actions::ActionResult::ToggleTerminal);

action!(toggle_debug_panel, {
	description: "Toggle debug panel",
	bindings: r#"normal "D""#,
}, |_ctx| evildoer_manifest::actions::ActionResult::ToggleDebugPanel);

result_handler!(
	RESULT_TOGGLE_PANEL_HANDLERS,
	TOGGLE_PANEL_BY_NAME,
	"toggle_panel_by_name",
	|result, ctx, _extend| {
		use evildoer_manifest::actions::ActionResult;
		use evildoer_manifest::editor_ctx::HandleOutcome;

		if let ActionResult::TogglePanel(name) = result {
			if let Some(ops) = ctx.buffer_ops() {
				ops.toggle_panel(name);
			}
			HandleOutcome::Handled
		} else {
			HandleOutcome::NotHandled
		}
	}
);

action!(focus_left, {
	description: "Focus split to the left",
	bindings: r#"window "h""#,
}, |_ctx| evildoer_manifest::actions::ActionResult::FocusLeft);

result_handler!(
	RESULT_FOCUS_LEFT_HANDLERS,
	HANDLE_FOCUS_LEFT,
	"focus_left",
	|r, ctx, _| {
		use evildoer_manifest::actions::ActionResult;
		use evildoer_manifest::editor_ctx::{HandleOutcome, MessageAccess};
		if matches!(r, ActionResult::FocusLeft) {
			if let Some(ops) = ctx.buffer_ops() {
				ops.focus_left();
			} else {
				ctx.notify("warning", "Buffer operations not available");
			}
		}
		HandleOutcome::Handled
	}
);

action!(focus_down, {
	description: "Focus split below",
	bindings: r#"window "j""#,
}, |_ctx| evildoer_manifest::actions::ActionResult::FocusDown);

result_handler!(
	RESULT_FOCUS_DOWN_HANDLERS,
	HANDLE_FOCUS_DOWN,
	"focus_down",
	|r, ctx, _| {
		use evildoer_manifest::actions::ActionResult;
		use evildoer_manifest::editor_ctx::{HandleOutcome, MessageAccess};
		if matches!(r, ActionResult::FocusDown) {
			if let Some(ops) = ctx.buffer_ops() {
				ops.focus_down();
			} else {
				ctx.notify("warning", "Buffer operations not available");
			}
		}
		HandleOutcome::Handled
	}
);

action!(focus_up, {
	description: "Focus split above",
	bindings: r#"window "k""#,
}, |_ctx| evildoer_manifest::actions::ActionResult::FocusUp);

result_handler!(
	RESULT_FOCUS_UP_HANDLERS,
	HANDLE_FOCUS_UP,
	"focus_up",
	|r, ctx, _| {
		use evildoer_manifest::actions::ActionResult;
		use evildoer_manifest::editor_ctx::{HandleOutcome, MessageAccess};
		if matches!(r, ActionResult::FocusUp) {
			if let Some(ops) = ctx.buffer_ops() {
				ops.focus_up();
			} else {
				ctx.notify("warning", "Buffer operations not available");
			}
		}
		HandleOutcome::Handled
	}
);

action!(focus_right, {
	description: "Focus split to the right",
	bindings: r#"window "l""#,
}, |_ctx| evildoer_manifest::actions::ActionResult::FocusRight);

result_handler!(
	RESULT_FOCUS_RIGHT_HANDLERS,
	HANDLE_FOCUS_RIGHT,
	"focus_right",
	|r, ctx, _| {
		use evildoer_manifest::actions::ActionResult;
		use evildoer_manifest::editor_ctx::{HandleOutcome, MessageAccess};
		if matches!(r, ActionResult::FocusRight) {
			if let Some(ops) = ctx.buffer_ops() {
				ops.focus_right();
			} else {
				ctx.notify("warning", "Buffer operations not available");
			}
		}
		HandleOutcome::Handled
	}
);

action!(buffer_next, {
	description: "Switch to next buffer",
	bindings: r#"window "n""#,
}, |_ctx| evildoer_manifest::actions::ActionResult::BufferNext);

result_handler!(
	RESULT_BUFFER_NEXT_HANDLERS,
	HANDLE_BUFFER_NEXT,
	"buffer_next",
	|r, ctx, _| {
		use evildoer_manifest::actions::ActionResult;
		use evildoer_manifest::editor_ctx::{HandleOutcome, MessageAccess};
		if matches!(r, ActionResult::BufferNext) {
			if let Some(ops) = ctx.buffer_ops() {
				ops.buffer_next();
			} else {
				ctx.notify("warning", "Buffer operations not available");
			}
		}
		HandleOutcome::Handled
	}
);

action!(buffer_prev, {
	description: "Switch to previous buffer",
	bindings: r#"window "p""#,
}, |_ctx| evildoer_manifest::actions::ActionResult::BufferPrev);

result_handler!(
	RESULT_BUFFER_PREV_HANDLERS,
	HANDLE_BUFFER_PREV,
	"buffer_prev",
	|r, ctx, _| {
		use evildoer_manifest::actions::ActionResult;
		use evildoer_manifest::editor_ctx::{HandleOutcome, MessageAccess};
		if matches!(r, ActionResult::BufferPrev) {
			if let Some(ops) = ctx.buffer_ops() {
				ops.buffer_prev();
			} else {
				ctx.notify("warning", "Buffer operations not available");
			}
		}
		HandleOutcome::Handled
	}
);

action!(close_split, {
	description: "Close current split",
	bindings: r#"window "q" "c""#,
}, |_ctx| evildoer_manifest::actions::ActionResult::CloseSplit);

result_handler!(
	RESULT_CLOSE_SPLIT_HANDLERS,
	HANDLE_CLOSE_SPLIT,
	"close_split",
	|r, ctx, _| {
		use evildoer_manifest::actions::ActionResult;
		use evildoer_manifest::editor_ctx::{HandleOutcome, MessageAccess};
		if matches!(r, ActionResult::CloseSplit) {
			if let Some(ops) = ctx.buffer_ops() {
				ops.close_split();
			} else {
				ctx.notify("warning", "Buffer operations not available");
			}
		}
		HandleOutcome::Handled
	}
);

action!(close_other_buffers, {
	description: "Close all other buffers",
	bindings: r#"window "o""#,
}, |_ctx| evildoer_manifest::actions::ActionResult::CloseOtherBuffers);

result_handler!(
	RESULT_CLOSE_OTHER_BUFFERS_HANDLERS,
	HANDLE_CLOSE_OTHER_BUFFERS,
	"close_other_buffers",
	|r, ctx, _| {
		use evildoer_manifest::actions::ActionResult;
		use evildoer_manifest::editor_ctx::{HandleOutcome, MessageAccess};
		if matches!(r, ActionResult::CloseOtherBuffers) {
			if let Some(ops) = ctx.buffer_ops() {
				ops.close_other_buffers();
			} else {
				ctx.notify("warning", "Buffer operations not available");
			}
		}
		HandleOutcome::Handled
	}
);
