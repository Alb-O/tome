//! File menu definitions.

use evildoer_manifest::{menu_group, menu_item};

menu_group!(file, {
	label: "File",
	priority: 0,
});

menu_item!(file_new, {
	group: "file",
	label: "New",
	command: "new",
	priority: 0,
});

menu_item!(file_open, {
	group: "file",
	label: "Open...",
	command: "open",
	priority: 10,
});

menu_item!(file_save, {
	group: "file",
	label: "Save",
	command: "write",
	priority: 20,
});

menu_item!(file_save_as, {
	group: "file",
	label: "Save As...",
	command: "write-to",
	priority: 30,
});

menu_item!(file_quit, {
	group: "file",
	label: "Quit",
	command: "quit",
	priority: 40,
});
