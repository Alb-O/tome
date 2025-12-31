//! Application menu bar.

use evildoer_tui::widgets::menu::{MenuEvent, MenuItem, MenuState};

/// Action triggered by menu item selection.
#[derive(Debug, Clone)]
pub enum MenuAction {
	Command(&'static str),
}

/// Creates the default application menu bar.
pub fn create_menu() -> MenuState<MenuAction> {
	MenuState::new(vec![
		MenuItem::group(
			"File",
			vec![
				MenuItem::item("New", MenuAction::Command("new")),
				MenuItem::item("Open...", MenuAction::Command("open")),
				MenuItem::item("Save", MenuAction::Command("write")),
				MenuItem::item("Save As...", MenuAction::Command("write-to")),
				MenuItem::item("Quit", MenuAction::Command("quit")),
			],
		),
		MenuItem::group(
			"Edit",
			vec![
				MenuItem::item("Undo", MenuAction::Command("undo")),
				MenuItem::item("Redo", MenuAction::Command("redo")),
				MenuItem::item("Cut", MenuAction::Command("cut")),
				MenuItem::item("Copy", MenuAction::Command("copy")),
				MenuItem::item("Paste", MenuAction::Command("paste")),
			],
		),
		MenuItem::group(
			"View",
			vec![
				MenuItem::item("Split Horizontal", MenuAction::Command("hsplit")),
				MenuItem::item("Split Vertical", MenuAction::Command("vsplit")),
				MenuItem::item("Close Split", MenuAction::Command("close")),
			],
		),
		MenuItem::group(
			"Help",
			vec![MenuItem::item("About", MenuAction::Command("about"))],
		),
	])
}

/// Processes menu events and queues corresponding commands.
pub fn process_menu_events(
	menu: &mut MenuState<MenuAction>,
	command_queue: &mut crate::editor::CommandQueue,
) {
	let mut had_selection = false;
	for event in menu.drain_events() {
		had_selection = true;
		match event {
			MenuEvent::Selected(MenuAction::Command(cmd)) => {
				command_queue.push(cmd, vec![]);
			}
		}
	}
	if had_selection {
		menu.reset();
	}
}
