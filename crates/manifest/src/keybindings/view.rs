//! Default keybindings for view mode.

use linkme::distributed_slice;
use tome_base::key::Key;

use crate::keybindings::{BindingMode, KEYBINDINGS_VIEW, KeyBindingDef};

const DEFAULT_PRIORITY: i16 = 100;

macro_rules! bind {
	($name:ident, $key:expr, $action:expr) => {
		#[distributed_slice(KEYBINDINGS_VIEW)]
		static $name: KeyBindingDef = KeyBindingDef {
			mode: BindingMode::View,
			key: $key,
			action: $action,
			priority: DEFAULT_PRIORITY,
		};
	};
}

bind!(KB_VIEW_J, Key::char('j'), "scroll_down");
bind!(KB_VIEW_K, Key::char('k'), "scroll_up");
