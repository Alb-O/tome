//! View menu definitions.

use evildoer_manifest::{menu_group, menu_item};

menu_group!(view, {
	label: "View",
	priority: 20,
});

menu_item!(view_split_horizontal, {
	group: "view",
	label: "Split Horizontal",
	command: "hsplit",
	priority: 0,
});

menu_item!(view_split_vertical, {
	group: "view",
	label: "Split Vertical",
	command: "vsplit",
	priority: 10,
});

menu_item!(view_close_split, {
	group: "view",
	label: "Close Split",
	command: "close",
	priority: 20,
});
