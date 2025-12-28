//! Find character actions (f/t/F/T commands).

use tome_base::key::Key;
use tome_manifest::actions::{ActionResult, PendingAction, PendingKind};
use tome_manifest::bound_action;

use crate::movement;

bound_action!(
	find_char,
	mode: Normal,
	key: Key::char('f'),
	description: "Select to next occurrence of character (inclusive)",
	|ctx| match ctx.args.char {
		Some(ch) => {
			let mut new_sel = ctx.selection.clone();
			new_sel.transform_mut(|r| {
				*r = movement::find_char_forward(ctx.text, *r, ch, ctx.count, true, ctx.extend);
			});
			ActionResult::Motion(new_sel)
		}
		None => ActionResult::Pending(PendingAction {
			kind: PendingKind::FindChar { inclusive: true },
			prompt: "find->".into(),
		}),
	}
);

bound_action!(
	find_char_to,
	mode: Normal,
	key: Key::char('t'),
	description: "Select to next occurrence of character (exclusive)",
	|ctx| match ctx.args.char {
		Some(ch) => {
			let mut new_sel = ctx.selection.clone();
			new_sel.transform_mut(|r| {
				*r = movement::find_char_forward(ctx.text, *r, ch, ctx.count, false, ctx.extend);
			});
			ActionResult::Motion(new_sel)
		}
		None => ActionResult::Pending(PendingAction {
			kind: PendingKind::FindChar { inclusive: false },
			prompt: "to->".into(),
		}),
	}
);

bound_action!(
	find_char_reverse,
	mode: Normal,
	key: Key::alt('f'),
	description: "Select to previous occurrence of character (inclusive)",
	|ctx| match ctx.args.char {
		Some(ch) => {
			let mut new_sel = ctx.selection.clone();
			new_sel.transform_mut(|r| {
				*r = movement::find_char_backward(ctx.text, *r, ch, ctx.count, true, ctx.extend);
			});
			ActionResult::Motion(new_sel)
		}
		None => ActionResult::Pending(PendingAction {
			kind: PendingKind::FindCharReverse { inclusive: true },
			prompt: "find<-".into(),
		}),
	}
);

bound_action!(
	find_char_to_reverse,
	mode: Normal,
	key: Key::alt('t'),
	description: "Select to previous occurrence of character (exclusive)",
	|ctx| match ctx.args.char {
		Some(ch) => {
			let mut new_sel = ctx.selection.clone();
			new_sel.transform_mut(|r| {
				*r = movement::find_char_backward(ctx.text, *r, ch, ctx.count, false, ctx.extend);
			});
			ActionResult::Motion(new_sel)
		}
		None => ActionResult::Pending(PendingAction {
			kind: PendingKind::FindCharReverse { inclusive: false },
			prompt: "to<-".into(),
		}),
	}
);
