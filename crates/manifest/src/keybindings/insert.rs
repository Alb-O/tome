//! Insert mode keybindings.

use linkme::distributed_slice;
use tome_base::key::{Key, SpecialKey};

use crate::keybindings::{BindingMode, KEYBINDINGS_INSERT, KeyBindingDef};

const DEFAULT_PRIORITY: i16 = 100;

macro_rules! bind {
	($name:ident, $key:expr, $action:expr) => {
		#[distributed_slice(KEYBINDINGS_INSERT)]
		static $name: KeyBindingDef = KeyBindingDef {
			mode: BindingMode::Insert,
			key: $key,
			action: $action,
			priority: DEFAULT_PRIORITY,
		};
	};
}

bind!(KB_INS_LEFT, Key::special(SpecialKey::Left), "move_left");
bind!(KB_INS_RIGHT, Key::special(SpecialKey::Right), "move_right");
bind!(KB_INS_UP, Key::special(SpecialKey::Up), "move_up_visual");
bind!(
	KB_INS_DOWN,
	Key::special(SpecialKey::Down),
	"move_down_visual"
);
bind!(
	KB_INS_HOME,
	Key::special(SpecialKey::Home),
	"move_line_start"
);
bind!(KB_INS_END, Key::special(SpecialKey::End), "move_line_end");

bind!(
	KB_INS_CTRL_LEFT,
	Key::special(SpecialKey::Left).with_ctrl(),
	"prev_word_start"
);
bind!(
	KB_INS_CTRL_RIGHT,
	Key::special(SpecialKey::Right).with_ctrl(),
	"next_word_start"
);

bind!(
	KB_INS_CTRL_HOME,
	Key::special(SpecialKey::Home).with_ctrl(),
	"document_start"
);
bind!(
	KB_INS_CTRL_END,
	Key::special(SpecialKey::End).with_ctrl(),
	"document_end"
);

bind!(
	KB_INS_PAGE_UP,
	Key::special(SpecialKey::PageUp),
	"scroll_page_up"
);
bind!(
	KB_INS_PAGE_DOWN,
	Key::special(SpecialKey::PageDown),
	"scroll_page_down"
);
