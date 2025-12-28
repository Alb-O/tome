//! Motion actions that wrap [`MotionDef`](tome_manifest::MotionDef) primitives.

use linkme::distributed_slice;
use tome_base::key::{Key, SpecialKey};
use tome_base::range::Range;
use tome_base::selection::Selection;
use tome_manifest::actions::{ActionContext, ActionDef, ActionResult};
use tome_manifest::keybindings::{BindingMode, KeyBindingDef};
use tome_manifest::{ACTIONS, find_motion};

/// Cursor movement - moves cursor (and all cursors) without creating new selections unless extending.
fn cursor_move_action(ctx: &ActionContext, motion_name: &str) -> ActionResult {
	let motion = match find_motion(motion_name) {
		Some(m) => m,
		None => return ActionResult::Error(format!("Unknown motion: {}", motion_name)),
	};

	let primary_index = ctx.selection.primary_index();

	// Move every selection head; when not extending, collapse to points at the new head.
	let new_ranges: Vec<Range> = ctx
		.selection
		.ranges()
		.iter()
		.map(|range| {
			let seed = if ctx.extend {
				*range
			} else {
				Range::point(range.head)
			};
			let moved = (motion.handler)(ctx.text, seed, ctx.count, ctx.extend);
			if ctx.extend {
				moved
			} else {
				Range::point(moved.head)
			}
		})
		.collect();

	ActionResult::Motion(Selection::from_vec(new_ranges, primary_index))
}

/// Selection-creating motion - creates new selection from old cursor to new position.
fn selection_motion_action(ctx: &ActionContext, motion_name: &str) -> ActionResult {
	let motion = match find_motion(motion_name) {
		Some(m) => m,
		None => return ActionResult::Error(format!("Unknown motion: {}", motion_name)),
	};

	// For selection-creating motions, we create a selection from cursor to new position
	if ctx.extend {
		// Extend each selection from its anchor using the detached cursor for the primary head
		let primary_index = ctx.selection.primary_index();
		let new_ranges: Vec<Range> = ctx
			.selection
			.ranges()
			.iter()
			.enumerate()
			.map(|(i, range)| {
				let seed = if i == primary_index {
					Range::new(range.anchor, ctx.cursor)
				} else {
					*range
				};
				(motion.handler)(ctx.text, seed, ctx.count, true)
			})
			.collect();
		ActionResult::Motion(Selection::from_vec(new_ranges, primary_index))
	} else {
		// Otherwise start fresh from cursor
		let current_range = Range::point(ctx.cursor);
		let new_range = (motion.handler)(ctx.text, current_range, ctx.count, false);
		ActionResult::Motion(Selection::single(new_range.anchor, new_range.head))
	}
}

/// Cursor action without keybindings (used when bindings come from elsewhere).
macro_rules! cursor_action {
	($name:ident, $motion:expr, $desc:expr) => {
		paste::paste! {
			fn [<handler_ $name>](ctx: &ActionContext) -> ActionResult {
				cursor_move_action(ctx, $motion)
			}

			#[distributed_slice(ACTIONS)]
			static [<ACTION_ $name:upper>]: ActionDef = ActionDef {
				id: concat!(env!("CARGO_PKG_NAME"), "::", stringify!($name)),
				name: stringify!($name),
				aliases: &[],
				description: $desc,
				handler: [<handler_ $name>],
				priority: 0,
				source: tome_manifest::RegistrySource::Crate(env!("CARGO_PKG_NAME")),
				required_caps: &[],
				flags: tome_manifest::flags::NONE,
			};
		}
	};
}

/// Cursor action with colocated multi-mode keybindings.
macro_rules! bound_cursor_action {
	($name:ident,
		motion: $motion:expr,
		description: $desc:expr,
		bindings: [$($mode:ident => [$($key:expr),+ $(,)?]),+ $(,)?] $(,)?
	) => {
		paste::paste! {
			fn [<handler_ $name>](ctx: &ActionContext) -> ActionResult {
				cursor_move_action(ctx, $motion)
			}

			#[distributed_slice(ACTIONS)]
			static [<ACTION_ $name:upper>]: ActionDef = ActionDef {
				id: concat!(env!("CARGO_PKG_NAME"), "::", stringify!($name)),
				name: stringify!($name),
				aliases: &[],
				description: $desc,
				handler: [<handler_ $name>],
				priority: 0,
				source: tome_manifest::RegistrySource::Crate(env!("CARGO_PKG_NAME")),
				required_caps: &[],
				flags: tome_manifest::flags::NONE,
			};

			$(bound_cursor_action!(@mode_keys $name, $mode, [] $($key),+);)+
		}
	};

	(@mode_keys $name:ident, $mode:ident, [$($done:tt)*] $key:expr $(, $rest:expr)+) => {
		paste::paste! {
			#[allow(non_upper_case_globals)]
			#[distributed_slice(tome_manifest::keybindings::[<KEYBINDINGS_ $mode:upper>])]
			static [<KB_ $name:upper _ $mode:upper _ $($done)*>]: KeyBindingDef =
				KeyBindingDef {
					mode: BindingMode::$mode,
					key: $key,
					action: stringify!($name),
					priority: 100,
				};
		}
		bound_cursor_action!(@mode_keys $name, $mode, [$($done)* _] $($rest),+);
	};
	(@mode_keys $name:ident, $mode:ident, [$($done:tt)*] $key:expr) => {
		paste::paste! {
			#[allow(non_upper_case_globals)]
			#[distributed_slice(tome_manifest::keybindings::[<KEYBINDINGS_ $mode:upper>])]
			static [<KB_ $name:upper _ $mode:upper _ $($done)*>]: KeyBindingDef =
				KeyBindingDef {
					mode: BindingMode::$mode,
					key: $key,
					action: stringify!($name),
					priority: 100,
				};
		}
	};
}

/// Selection-creating action with colocated multi-mode keybindings.
macro_rules! bound_selection_action {
	($name:ident,
		motion: $motion:expr,
		description: $desc:expr,
		bindings: [$($mode:ident => [$($key:expr),+ $(,)?]),+ $(,)?] $(,)?
	) => {
		paste::paste! {
			fn [<handler_ $name>](ctx: &ActionContext) -> ActionResult {
				selection_motion_action(ctx, $motion)
			}

			#[distributed_slice(ACTIONS)]
			static [<ACTION_ $name:upper>]: ActionDef = ActionDef {
				id: concat!(env!("CARGO_PKG_NAME"), "::", stringify!($name)),
				name: stringify!($name),
				aliases: &[],
				description: $desc,
				handler: [<handler_ $name>],
				priority: 0,
				source: tome_manifest::RegistrySource::Crate(env!("CARGO_PKG_NAME")),
				required_caps: &[],
				flags: tome_manifest::flags::NONE,
			};

			$(bound_selection_action!(@mode_keys $name, $mode, [] $($key),+);)+
		}
	};

	(@mode_keys $name:ident, $mode:ident, [$($done:tt)*] $key:expr $(, $rest:expr)+) => {
		paste::paste! {
			#[allow(non_upper_case_globals)]
			#[distributed_slice(tome_manifest::keybindings::[<KEYBINDINGS_ $mode:upper>])]
			static [<KB_ $name:upper _ $mode:upper _ $($done)*>]: KeyBindingDef =
				KeyBindingDef {
					mode: BindingMode::$mode,
					key: $key,
					action: stringify!($name),
					priority: 100,
				};
		}
		bound_selection_action!(@mode_keys $name, $mode, [$($done)* _] $($rest),+);
	};
	(@mode_keys $name:ident, $mode:ident, [$($done:tt)*] $key:expr) => {
		paste::paste! {
			#[allow(non_upper_case_globals)]
			#[distributed_slice(tome_manifest::keybindings::[<KEYBINDINGS_ $mode:upper>])]
			static [<KB_ $name:upper _ $mode:upper _ $($done)*>]: KeyBindingDef =
				KeyBindingDef {
					mode: BindingMode::$mode,
					key: $key,
					action: stringify!($name),
					priority: 100,
				};
		}
	};
}

bound_cursor_action!(
	move_left,
	motion: "move_left",
	description: "Move left",
	bindings: [
		Normal => [Key::char('h'), Key::special(SpecialKey::Left)],
		Insert => [Key::special(SpecialKey::Left)],
	],
);

bound_cursor_action!(
	move_right,
	motion: "move_right",
	description: "Move right",
	bindings: [
		Normal => [Key::char('l'), Key::special(SpecialKey::Right)],
		Insert => [Key::special(SpecialKey::Right)],
	],
);

cursor_action!(move_up, "move_up", "Move up");
cursor_action!(move_down, "move_down", "Move down");

bound_cursor_action!(
	move_line_start,
	motion: "line_start",
	description: "Move to line start",
	bindings: [
		Normal => [Key::char('0'), Key::special(SpecialKey::Home), Key::alt('h')],
		Goto => [Key::char('h')],
		Insert => [Key::special(SpecialKey::Home)],
	],
);

bound_cursor_action!(
	move_line_end,
	motion: "line_end",
	description: "Move to line end",
	bindings: [
		Normal => [Key::char('$'), Key::special(SpecialKey::End), Key::alt('l')],
		Goto => [Key::char('l')],
		Insert => [Key::special(SpecialKey::End)],
	],
);

bound_cursor_action!(
	move_first_nonblank,
	motion: "first_nonwhitespace",
	description: "Move to first non-blank",
	bindings: [
		Normal => [Key::char('^')],
		Goto => [Key::char('i')],
	],
);

bound_cursor_action!(
	document_start,
	motion: "document_start",
	description: "Move to document start",
	bindings: [
		Normal => [Key::special(SpecialKey::Home).with_ctrl()],
		Goto => [Key::char('g'), Key::char('k')],
		Insert => [Key::special(SpecialKey::Home).with_ctrl()],
	],
);

bound_cursor_action!(
	document_end,
	motion: "document_end",
	description: "Move to document end",
	bindings: [
		Normal => [Key::char('G'), Key::special(SpecialKey::End).with_ctrl()],
		Goto => [Key::char('j'), Key::char('e')],
		Insert => [Key::special(SpecialKey::End).with_ctrl()],
	],
);

// Selection-creating motions - create selections
bound_selection_action!(
	next_word_start,
	motion: "next_word_start",
	description: "Move to next word start",
	bindings: [
		Normal => [Key::char('w')],
		Insert => [Key::special(SpecialKey::Right).with_ctrl()],
	],
);

bound_selection_action!(
	next_word_end,
	motion: "next_word_end",
	description: "Move to next word end",
	bindings: [Normal => [Key::char('e')]],
);

bound_selection_action!(
	prev_word_start,
	motion: "prev_word_start",
	description: "Move to previous word start",
	bindings: [
		Normal => [Key::char('b')],
		Insert => [Key::special(SpecialKey::Left).with_ctrl()],
	],
);

bound_selection_action!(
	prev_word_end,
	motion: "prev_word_end",
	description: "Move to previous word end",
	bindings: [Normal => [Key::alt('e')]],
);

bound_selection_action!(
	next_long_word_start,
	motion: "next_long_word_start",
	description: "Move to next WORD start",
	bindings: [Normal => [Key::char('W'), Key::alt('w')]],
);

bound_selection_action!(
	next_long_word_end,
	motion: "next_long_word_end",
	description: "Move to next WORD end",
	bindings: [Normal => [Key::char('E')]],
);

bound_selection_action!(
	prev_long_word_start,
	motion: "prev_long_word_start",
	description: "Move to previous WORD start",
	bindings: [Normal => [Key::char('B'), Key::alt('b')]],
);
