//! Scroll/view actions.

use tome_manifest::actions::{ActionResult, EditAction, ScrollAmount, ScrollDir, VisualDirection};

use crate::action;

action!(scroll_up, { description: "Scroll view up" }, |ctx| {
	ActionResult::Edit(EditAction::Scroll {
		direction: ScrollDir::Up,
		amount: ScrollAmount::Line(ctx.count),
		extend: ctx.extend,
	})
});

action!(scroll_down, { description: "Scroll view down" }, |ctx| {
	ActionResult::Edit(EditAction::Scroll {
		direction: ScrollDir::Down,
		amount: ScrollAmount::Line(ctx.count),
		extend: ctx.extend,
	})
});

action!(scroll_half_page_up, { description: "Scroll half page up" }, |ctx| {
	ActionResult::Edit(EditAction::Scroll {
		direction: ScrollDir::Up,
		amount: ScrollAmount::HalfPage,
		extend: ctx.extend,
	})
});

action!(scroll_half_page_down, { description: "Scroll half page down" }, |ctx| {
	ActionResult::Edit(EditAction::Scroll {
		direction: ScrollDir::Down,
		amount: ScrollAmount::HalfPage,
		extend: ctx.extend,
	})
});

action!(scroll_page_up, { description: "Scroll page up" }, |ctx| {
	ActionResult::Edit(EditAction::Scroll {
		direction: ScrollDir::Up,
		amount: ScrollAmount::FullPage,
		extend: ctx.extend,
	})
});

action!(scroll_page_down, { description: "Scroll page down" }, |ctx| {
	ActionResult::Edit(EditAction::Scroll {
		direction: ScrollDir::Down,
		amount: ScrollAmount::FullPage,
		extend: ctx.extend,
	})
});

action!(move_up_visual, { description: "Move up (visual lines)" }, |ctx| {
	ActionResult::Edit(EditAction::MoveVisual {
		direction: VisualDirection::Up,
		count: ctx.count,
		extend: ctx.extend,
	})
});

action!(move_down_visual, { description: "Move down (visual lines)" }, |ctx| {
	ActionResult::Edit(EditAction::MoveVisual {
		direction: VisualDirection::Down,
		count: ctx.count,
		extend: ctx.extend,
	})
});
