//! Edit menu definitions.

use evildoer_manifest::{menu_group, menu_item};

menu_group!(edit, {
	label: "Edit",
	priority: 10,
});

menu_item!(edit_undo, {
	group: "edit",
	label: "Undo",
	command: "undo",
	priority: 0,
});

menu_item!(edit_redo, {
	group: "edit",
	label: "Redo",
	command: "redo",
	priority: 10,
});

menu_item!(edit_cut, {
	group: "edit",
	label: "Cut",
	command: "cut",
	priority: 20,
});

menu_item!(edit_copy, {
	group: "edit",
	label: "Copy",
	command: "copy",
	priority: 30,
});

menu_item!(edit_paste, {
	group: "edit",
	label: "Paste",
	command: "paste",
	priority: 40,
});
