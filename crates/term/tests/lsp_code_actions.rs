//! LSP code actions integration tests using kitty harness.
//!
//! Tests code actions popup, lightbulb gutter indicator, and applying quickfixes.
//! Requires: LSP_TESTS=1 KITTY_TESTS=1 DISPLAY=:0

mod helpers;
mod lsp_helpers;

use std::time::Duration;

use helpers::type_chars;
use kitty_test_harness::{
	kitty_send_keys, pause_briefly, run_with_timeout, wait_for_screen_text_clean,
	with_kitty_capture,
};
use lsp_helpers::{
	debug_screen, fixtures_dir, require_lsp_tests, wait_for_lsp_ready, workspace_dir,
	xeno_cmd_with_file,
};
use termwiz::input::KeyCode;

const TEST_TIMEOUT: Duration = Duration::from_secs(60);
const LSP_INIT_TIMEOUT: Duration = Duration::from_secs(15);
const LIGHTBULB_ICON: &str = "\u{f0335}";

/// Code actions popup appears for diagnostics.
///
/// User story: "As a user, pressing `ga` (or `<space>a`) shows available actions"
#[serial_test::serial]
#[test]
fn code_actions_quickfix_popup() {
	if !require_lsp_tests() {
		return;
	}

	run_with_timeout(TEST_TIMEOUT, || {
		let fixture_file = fixtures_dir().join("rust-basic/src/main.rs");
		let cmd = xeno_cmd_with_file(&fixture_file.display().to_string());

		with_kitty_capture(&workspace_dir(), &cmd, |kitty| {
			wait_for_lsp_ready(kitty, LSP_INIT_TIMEOUT);
			std::thread::sleep(Duration::from_secs(3));

			debug_screen(kitty, "INITIAL STATE");

			// Search for the unused variable line
			kitty_send_keys!(kitty, KeyCode::Char('/'));
			pause_briefly();
			debug_screen(kitty, "AFTER / (SEARCH PROMPT)");

			type_chars(kitty, "unused_var");
			pause_briefly();
			debug_screen(kitty, "AFTER TYPING unused_var");

			kitty_send_keys!(kitty, KeyCode::Enter);
			kitty_send_keys!(kitty, KeyCode::Escape);
			pause_briefly();
			debug_screen(kitty, "AFTER SEARCH (CURSOR ON unused_var)");

			// Trigger code actions popup
			kitty_send_keys!(kitty, KeyCode::Char(' '));
			kitty_send_keys!(kitty, KeyCode::Char('a'));
			pause_briefly();
			debug_screen(kitty, "AFTER SPACE+A (CODE ACTIONS)");

			let (_raw, clean) =
				wait_for_screen_text_clean(kitty, Duration::from_secs(5), |_raw, clean| {
					clean.contains("Remove")
						|| clean.contains("prefix")
						|| clean.contains("unused")
						|| clean.contains("quickfix")
				});

			assert!(
				clean.contains("Remove")
					|| clean.contains("prefix")
					|| clean.contains("unused")
					|| clean.contains("quickfix"),
				"Expected code actions popup with quickfix suggestions. Screen:\n{clean}"
			);
		});
	});
}

/// Lightbulb appears in the gutter when code actions are available.
///
/// User story: "As a user, I see a lightbulb when code actions are available"
#[serial_test::serial]
#[test]
fn code_actions_lightbulb() {
	if !require_lsp_tests() {
		return;
	}

	run_with_timeout(TEST_TIMEOUT, || {
		let fixture_file = fixtures_dir().join("rust-basic/src/main.rs");
		let cmd = xeno_cmd_with_file(&fixture_file.display().to_string());

		with_kitty_capture(&workspace_dir(), &cmd, |kitty| {
			wait_for_lsp_ready(kitty, LSP_INIT_TIMEOUT);
			std::thread::sleep(Duration::from_secs(3));

			// Navigate to the unused variable line to ensure it's visible
			kitty_send_keys!(kitty, KeyCode::Char('/'));
			pause_briefly();
			debug_screen(kitty, "AFTER / (SEARCH PROMPT)");

			type_chars(kitty, "unused_var");
			pause_briefly();
			debug_screen(kitty, "AFTER TYPING unused_var");

			kitty_send_keys!(kitty, KeyCode::Enter);
			kitty_send_keys!(kitty, KeyCode::Escape);
			pause_briefly();

			debug_screen(kitty, "AFTER SEARCH (CHECK LIGHTBULB)");

			let (_raw, clean) =
				wait_for_screen_text_clean(kitty, Duration::from_secs(5), |_raw, clean| {
					clean.contains(LIGHTBULB_ICON)
				});

			assert!(
				clean.contains(LIGHTBULB_ICON),
				"Expected lightbulb icon in gutter when code actions are available. Screen:\n{clean}"
			);
		});
	});
}

/// Accepting a code action applies the quickfix.
///
/// User story: "As a user, selecting a quickfix applies the change"
#[serial_test::serial]
#[test]
fn code_actions_apply_quickfix() {
	if !require_lsp_tests() {
		return;
	}

	run_with_timeout(TEST_TIMEOUT, || {
		let fixture_file = fixtures_dir().join("rust-basic/src/main.rs");
		let cmd = xeno_cmd_with_file(&fixture_file.display().to_string());

		with_kitty_capture(&workspace_dir(), &cmd, |kitty| {
			wait_for_lsp_ready(kitty, LSP_INIT_TIMEOUT);
			std::thread::sleep(Duration::from_secs(3));

			// Navigate to the unused variable line
			kitty_send_keys!(kitty, KeyCode::Char('/'));
			pause_briefly();
			debug_screen(kitty, "AFTER / (SEARCH PROMPT)");

			type_chars(kitty, "unused_var");
			pause_briefly();
			debug_screen(kitty, "AFTER TYPING unused_var");

			kitty_send_keys!(kitty, KeyCode::Enter);
			kitty_send_keys!(kitty, KeyCode::Escape);
			pause_briefly();

			let (_raw, before_apply) =
				wait_for_screen_text_clean(kitty, Duration::from_secs(3), |_raw, clean| {
					clean.contains("unused_var")
				});

			assert!(
				before_apply.contains("unused_var"),
				"Expected unused_var line to be visible before applying action. Screen:\n{before_apply}"
			);

			debug_screen(kitty, "BEFORE CODE ACTIONS APPLY");

			// Trigger code actions popup
			kitty_send_keys!(kitty, KeyCode::Char(' '));
			kitty_send_keys!(kitty, KeyCode::Char('a'));
			pause_briefly();
			debug_screen(kitty, "CODE ACTIONS POPUP OPEN");

			// Accept the first code action
			kitty_send_keys!(kitty, KeyCode::Enter);
			pause_briefly();
			pause_briefly();

			let (_raw, after_apply) =
				wait_for_screen_text_clean(kitty, Duration::from_secs(5), |_raw, clean| {
					clean.contains("_unused_var") || !clean.contains("let unused_var = 42")
				});

			debug_screen(kitty, "AFTER CODE ACTION APPLIED");

			let has_prefixed = after_apply.contains("_unused_var");
			let has_original = after_apply.contains("let unused_var = 42");
			assert!(
				has_prefixed || !has_original,
				"Expected code action to update or remove unused_var line. Screen:\n{after_apply}"
			);
		});
	});
}
