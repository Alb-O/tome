//! Help menu definitions.

use evildoer_manifest::{menu_group, menu_item};

menu_group!(help, {
	label: "Help",
	priority: 100,
});

menu_item!(help_about, {
	group: "help",
	label: "About",
	command: "about",
	priority: 0,
});
