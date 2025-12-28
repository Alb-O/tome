//! Default keybindings for normal mode.

use linkme::distributed_slice;
use tome_base::key::{Key, SpecialKey};

use crate::keybindings::{BindingMode, KEYBINDINGS_NORMAL, KeyBindingDef};

const DEFAULT_PRIORITY: i16 = 100;

macro_rules! bind {
	($name:ident, $key:expr, $action:expr) => {
		#[distributed_slice(KEYBINDINGS_NORMAL)]
		static $name: KeyBindingDef = KeyBindingDef {
			mode: BindingMode::Normal,
			key: $key,
			action: $action,
			priority: DEFAULT_PRIORITY,
		};
	};
}

bind!(KB_H, Key::char('h'), "move_left");
bind!(KB_L, Key::char('l'), "move_right");
bind!(KB_J, Key::char('j'), "move_down_visual");
bind!(KB_K, Key::char('k'), "move_up_visual");
bind!(KB_LEFT, Key::special(SpecialKey::Left), "move_left");
bind!(KB_RIGHT, Key::special(SpecialKey::Right), "move_right");
bind!(KB_DOWN, Key::special(SpecialKey::Down), "move_down_visual");
bind!(KB_UP, Key::special(SpecialKey::Up), "move_up_visual");
bind!(KB_HOME, Key::special(SpecialKey::Home), "move_line_start");
bind!(KB_END, Key::special(SpecialKey::End), "move_line_end");
bind!(
	KB_HOME_CTRL,
	Key::special(SpecialKey::Home).with_ctrl(),
	"document_start"
);
bind!(
	KB_END_CTRL,
	Key::special(SpecialKey::End).with_ctrl(),
	"document_end"
);
bind!(
	KB_PAGE_UP,
	Key::special(SpecialKey::PageUp),
	"scroll_page_up"
);
bind!(
	KB_PAGE_DOWN,
	Key::special(SpecialKey::PageDown),
	"scroll_page_down"
);

bind!(KB_W, Key::char('w'), "next_word_start");
bind!(KB_B, Key::char('b'), "prev_word_start");
bind!(KB_E, Key::char('e'), "next_word_end");

bind!(KB_W_UPPER, Key::char('W'), "next_long_word_start");
bind!(KB_B_UPPER, Key::char('B'), "prev_long_word_start");
bind!(KB_E_UPPER, Key::char('E'), "next_long_word_end");
bind!(KB_W_ALT, Key::alt('w'), "next_long_word_start");
bind!(KB_B_ALT, Key::alt('b'), "prev_long_word_start");
bind!(KB_E_ALT, Key::alt('e'), "next_long_word_end");

bind!(KB_0, Key::char('0'), "move_line_start");
bind!(KB_CARET, Key::char('^'), "move_first_nonblank");
bind!(KB_DOLLAR, Key::char('$'), "move_line_end");
bind!(KB_H_ALT, Key::alt('h'), "move_line_start");
bind!(KB_L_ALT, Key::alt('l'), "move_line_end");

bind!(KB_GG, Key::char('g'), "goto_mode");
bind!(KB_G_UPPER, Key::char('G'), "document_end");

bind!(KB_D, Key::char('d'), "delete");
bind!(KB_D_ALT, Key::alt('d'), "delete_no_yank");
bind!(KB_C, Key::char('c'), "change");
bind!(KB_C_ALT, Key::alt('c'), "change_no_yank");
bind!(KB_Y, Key::char('y'), "yank");
bind!(KB_P, Key::char('p'), "paste_after");
bind!(KB_P_UPPER, Key::char('P'), "paste_before");
bind!(KB_P_ALT, Key::alt('p'), "paste_all_after");
bind!(KB_P_ALT_UPPER, Key::alt('P'), "paste_all_before");

bind!(KB_U, Key::char('u'), "undo");
bind!(KB_U_UPPER, Key::char('U'), "redo");

bind!(KB_I, Key::char('i'), "insert_before");
bind!(KB_A, Key::char('a'), "insert_after");
bind!(KB_I_UPPER, Key::char('I'), "insert_line_start");
bind!(KB_A_UPPER, Key::char('A'), "insert_line_end");
bind!(KB_O, Key::char('o'), "open_below");
bind!(KB_O_UPPER, Key::char('O'), "open_above");
bind!(KB_O_ALT, Key::alt('o'), "add_line_below");
bind!(KB_O_ALT_UPPER, Key::alt('O'), "add_line_above");

// Selection ops keybindings are colocated with their actions in
// tome-stdlib/src/actions/selection_ops.rs using bound_action! macro.

bind!(KB_GT, Key::char('>'), "indent");
bind!(KB_LT, Key::char('<'), "deindent");

bind!(KB_BACKTICK, Key::char('`'), "to_lowercase");
bind!(KB_TILDE, Key::char('~'), "to_uppercase");
bind!(KB_BACKTICK_ALT, Key::alt('`'), "swap_case");

bind!(KB_J_ALT, Key::alt('j'), "join_lines");

bind!(KB_CTRL_U, Key::ctrl('u'), "scroll_half_page_up");
bind!(KB_CTRL_D, Key::ctrl('d'), "scroll_half_page_down");
bind!(KB_CTRL_B, Key::ctrl('b'), "scroll_page_up");
bind!(KB_CTRL_F, Key::ctrl('f'), "scroll_page_down");

bind!(KB_V, Key::char('v'), "view_mode");

bind!(KB_F, Key::char('f'), "find_char");
bind!(KB_T, Key::char('t'), "find_char_to");
bind!(KB_F_ALT, Key::alt('f'), "find_char_reverse");
bind!(KB_T_ALT, Key::alt('t'), "find_char_to_reverse");

bind!(KB_R, Key::char('r'), "replace_char");

bind!(KB_ALT_I, Key::alt('i'), "select_object_inner");
bind!(KB_ALT_A, Key::alt('a'), "select_object_around");
bind!(KB_BRACKET_OPEN, Key::char('['), "select_object_to_start");
bind!(KB_BRACKET_CLOSE, Key::char(']'), "select_object_to_end");
bind!(KB_BRACE_OPEN, Key::char('{'), "select_object_to_start");
bind!(KB_BRACE_CLOSE, Key::char('}'), "select_object_to_end");

// Duplicate/merge selection keybindings are colocated with their actions in
// tome-stdlib/src/actions/selection_ops.rs using bound_action! macro.

bind!(KB_AMP_ALIGN, Key::char('&'), "align");
bind!(KB_AMP_ALT_COPY_INDENT, Key::alt('&'), "copy_indent");
bind!(KB_AT_TABS, Key::char('@'), "tabs_to_spaces");
bind!(KB_AT_ALT_SPACES, Key::alt('@'), "spaces_to_tabs");
bind!(KB_UNDERSCORE_TRIM, Key::char('_'), "trim_selections");

bind!(KB_CTRL_W, Key::ctrl('w'), "window_mode");
