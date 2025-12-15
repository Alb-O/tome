//! Default keybindings for normal mode.

use linkme::distributed_slice;

use crate::ext::keybindings::{BindingMode, KeyBindingDef, KEYBINDINGS};
use crate::key::Key;

const DEFAULT_PRIORITY: i16 = 100;

macro_rules! bind {
    ($name:ident, $key:expr, $action:expr) => {
        #[distributed_slice(KEYBINDINGS)]
        static $name: KeyBindingDef = KeyBindingDef {
            mode: BindingMode::Normal,
            key: $key,
            action: $action,
            priority: DEFAULT_PRIORITY,
        };
    };
}

// Basic movement
bind!(KB_H, Key::char('h'), "move_left");
bind!(KB_L, Key::char('l'), "move_right");
// Note: j/k use legacy Command::MoveUp/MoveDown for visual line wrapping support

// Word movement
bind!(KB_W, Key::char('w'), "next_word_start");
bind!(KB_B, Key::char('b'), "prev_word_start");
bind!(KB_E, Key::char('e'), "next_word_end");

// WORD movement
bind!(KB_W_UPPER, Key::char('W'), "next_long_word_start");
bind!(KB_B_UPPER, Key::char('B'), "prev_long_word_start");
bind!(KB_E_UPPER, Key::char('E'), "next_long_word_end");

// Line movement
bind!(KB_0, Key::char('0'), "move_line_start");
bind!(KB_CARET, Key::char('^'), "move_first_nonblank");
bind!(KB_DOLLAR, Key::char('$'), "move_line_end");

// Document movement
bind!(KB_GG, Key::char('g'), "goto_mode");
bind!(KB_G_UPPER, Key::char('G'), "document_end");
