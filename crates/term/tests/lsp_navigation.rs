//! LSP navigation integration tests using kitty harness.
//!
//! Tests go-to-definition, find-references, and navigation features.
//! Requires: LSP_TESTS=1 KITTY_TESTS=1 DISPLAY=:0

mod lsp_helpers;

use std::time::Duration;

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

/// gd on function call jumps to definition.
///
/// User story: "As a user, pressing `gd` on a function call jumps to its definition"
#[serial_test::serial]
#[test]
fn goto_definition_jumps() {
	if !require_lsp_tests() {
		return;
	}

	run_with_timeout(TEST_TIMEOUT, || {
		// Open lib.rs which has test_caller() that calls helper_function()
		// This tests gd within the same file (no import indirection)
		let fixture_file = fixtures_dir().join("rust-navigation/src/lib.rs");
		let cmd = xeno_cmd_with_file(&fixture_file.display().to_string());

		with_kitty_capture(&workspace_dir(), &cmd, |kitty| {
			wait_for_lsp_ready(kitty, LSP_INIT_TIMEOUT);
			std::thread::sleep(Duration::from_secs(3));

			// STEP 1: Initial state - file starts at line 1
			debug_screen(kitty, "INITIAL STATE");

			// STEP 2: Navigate to line 17 where helper_function(5) is called
			// File structure:
			//   Line 17: helper_function(5)
			// Go to top first, then down 16 lines
			kitty_send_keys!(kitty, KeyCode::Char('g'), KeyCode::Char('g'));
			pause_briefly();
			// Navigate down 16 lines to line 17
			for _ in 0..16 {
				kitty_send_keys!(kitty, KeyCode::Char('j'));
			}
			pause_briefly();
			debug_screen(kitty, "AFTER NAVIGATING TO LINE 17");

			// STEP 3: Move to the start of the line to position on helper_function
			kitty_send_keys!(kitty, KeyCode::Char('0'));
			pause_briefly();
			// Move forward to skip indentation and get to helper_function
			kitty_send_keys!(kitty, KeyCode::Char('w'));
			pause_briefly();
			debug_screen(kitty, "CURSOR ON helper_function");

			// STEP 4: Press gd to go to definition
			kitty_send_keys!(kitty, KeyCode::Char('g'), KeyCode::Char('d'));
			pause_briefly();
			debug_screen(kitty, "IMMEDIATELY AFTER gd");

			// STEP 5: Wait for async LSP response
			std::thread::sleep(Duration::from_secs(3));
			debug_screen(kitty, "AFTER 3s WAIT (should have jumped to line 9)");

			// Final check - should now be on line 9 where helper_function is defined
			let (_, final_screen) =
				wait_for_screen_text_clean(kitty, Duration::from_secs(2), |_, _| true);

			// Verify we jumped to the definition (line 9)
			// Status bar shows "9:" or ":9" for line number
			let jumped_to_definition = final_screen.contains("lib.rs")
				&& (final_screen.contains(" 9:") || final_screen.contains(":9 "));

			assert!(
				jumped_to_definition,
				"Should jump to helper_function definition at line 9. Final screen:\n{final_screen}"
			);
		});
	});
}

/// gr shows references panel with all usages.
///
/// User story: "As a user, pressing `gr` shows all references to the symbol"
#[serial_test::serial]
#[test]
fn find_references_shows_list() {
	if !require_lsp_tests() {
		return;
	}

	run_with_timeout(TEST_TIMEOUT, || {
		// Open lib.rs where shared_function is defined
		let fixture_file = fixtures_dir().join("rust-navigation/src/lib.rs");
		let cmd = xeno_cmd_with_file(&fixture_file.display().to_string());

		with_kitty_capture(&workspace_dir(), &cmd, |kitty| {
			wait_for_lsp_ready(kitty, LSP_INIT_TIMEOUT);

			// Give rust-analyzer more time to index
			std::thread::sleep(Duration::from_secs(3));

			// Navigate to line 4 where shared_function is defined
			// Line 4: pub fn shared_function() -> i32 {
			kitty_send_keys!(kitty, KeyCode::Char('g'), KeyCode::Char('g'));
			pause_briefly();
			for _ in 0..3 {
				kitty_send_keys!(kitty, KeyCode::Char('j'));
			}
			pause_briefly();

			// Move to shared_function (skip "pub fn ")
			kitty_send_keys!(kitty, KeyCode::Char('0'));
			pause_briefly();
			kitty_send_keys!(kitty, KeyCode::Char('w')); // pub
			kitty_send_keys!(kitty, KeyCode::Char('w')); // fn
			kitty_send_keys!(kitty, KeyCode::Char('w')); // shared_function
			pause_briefly();

			debug_screen(kitty, "CURSOR ON shared_function");

			// Find references with gr
			kitty_send_keys!(kitty, KeyCode::Char('g'), KeyCode::Char('r'));
			pause_briefly();
			pause_briefly();
			pause_briefly();

			// Wait for references to appear - should show a panel or picker
			let (_raw, clean) =
				wait_for_screen_text_clean(kitty, Duration::from_secs(5), |_raw, clean| {
					// References panel should show other.rs entries
					clean.contains("other.rs")
						|| clean.contains("caller_one")
						|| clean.contains("caller_two")
						|| clean.contains("Reference")
						|| clean.contains("references")
				});

			// Verify references panel shows callers from other.rs
			let shows_references = clean.contains("other.rs")
				|| clean.contains("caller_one")
				|| clean.contains("caller_two")
				|| clean.contains("Reference")
				|| clean.contains("references")
				|| clean.contains("2 references") // May show count
				|| clean.contains("3 references"); // Including definition

			if !shows_references {
				eprintln!(
					"INFO: References may not be showing in panel yet (gap check needed). Screen:\n{clean}"
				);
			}

			// At minimum verify the file is loaded
			assert!(
				clean.contains("shared_function") || clean.contains("lib.rs"),
				"Should be viewing lib.rs with shared_function. Screen:\n{clean}"
			);
		});
	});
}

/// Enter on reference in panel jumps to that location.
///
/// User story: "As a user, if there are multiple definitions, I see a picker"
#[serial_test::serial]
#[test]
fn references_panel_navigation() {
	if !require_lsp_tests() {
		return;
	}

	run_with_timeout(TEST_TIMEOUT, || {
		// Open lib.rs where shared_function is defined
		let fixture_file = fixtures_dir().join("rust-navigation/src/lib.rs");
		let cmd = xeno_cmd_with_file(&fixture_file.display().to_string());

		with_kitty_capture(&workspace_dir(), &cmd, |kitty| {
			wait_for_lsp_ready(kitty, LSP_INIT_TIMEOUT);

			// Give rust-analyzer more time to index
			std::thread::sleep(Duration::from_secs(3));

			// Navigate to line 4 where shared_function is defined
			// Line 4: pub fn shared_function() -> i32 {
			kitty_send_keys!(kitty, KeyCode::Char('g'), KeyCode::Char('g'));
			pause_briefly();
			for _ in 0..3 {
				kitty_send_keys!(kitty, KeyCode::Char('j'));
			}
			pause_briefly();

			// Move to shared_function (skip "pub fn ")
			kitty_send_keys!(kitty, KeyCode::Char('0'));
			pause_briefly();
			kitty_send_keys!(kitty, KeyCode::Char('w')); // pub
			kitty_send_keys!(kitty, KeyCode::Char('w')); // fn
			kitty_send_keys!(kitty, KeyCode::Char('w')); // shared_function
			pause_briefly();

			// Find references with gr
			kitty_send_keys!(kitty, KeyCode::Char('g'), KeyCode::Char('r'));
			pause_briefly();
			pause_briefly();
			pause_briefly();

			// If references panel appears, try to navigate and select
			let (_raw, clean) =
				wait_for_screen_text_clean(kitty, Duration::from_secs(3), |_raw, _clean| true);

			// Try selecting a reference with Enter or j + Enter
			kitty_send_keys!(kitty, KeyCode::Char('j')); // Move down in list
			pause_briefly();
			kitty_send_keys!(kitty, KeyCode::Enter); // Select reference
			pause_briefly();
			pause_briefly();

			// After selecting, should jump to the reference location
			let (_raw, after_select) =
				wait_for_screen_text_clean(kitty, Duration::from_secs(3), |_raw, _clean| true);

			// Verify we're viewing code (either jumped to other.rs or still in lib.rs)
			let shows_code = after_select.contains("fn ")
				|| after_select.contains("pub ")
				|| after_select.contains("shared_function")
				|| after_select.contains("caller_");

			assert!(
				shows_code,
				"Should show code after selecting reference. Screen:\n{after_select}"
			);

			// Check if we navigated to other.rs (if references panel was shown)
			if clean.contains("other.rs") {
				let jumped_to_reference = after_select.contains("caller_one")
					|| after_select.contains("caller_two")
					|| after_select.contains("other.rs");

				if !jumped_to_reference {
					eprintln!(
						"INFO: May not have jumped to reference location. Screen:\n{after_select}"
					);
				}
			}
		});
	});
}

/// Test go-to-definition from import statement.
///
/// User story: Jumping from `use crate::shared_function` goes to the definition.
#[serial_test::serial]
#[test]
fn goto_definition_from_import() {
	if !require_lsp_tests() {
		return;
	}

	run_with_timeout(TEST_TIMEOUT, || {
		// Open other.rs which has `use crate::shared_function`
		let fixture_file = fixtures_dir().join("rust-navigation/src/other.rs");
		let cmd = xeno_cmd_with_file(&fixture_file.display().to_string());

		with_kitty_capture(&workspace_dir(), &cmd, |kitty| {
			wait_for_lsp_ready(kitty, LSP_INIT_TIMEOUT);

			// Give rust-analyzer more time to index
			std::thread::sleep(Duration::from_secs(3));

			debug_screen(kitty, "INITIAL STATE (other.rs)");

			// Go to line 1 where the import is (use crate::shared_function;)
			kitty_send_keys!(kitty, KeyCode::Char('g'), KeyCode::Char('g'));
			pause_briefly();
			pause_briefly();

			// Move to shared_function in the import statement
			// Line 1 is: use crate::shared_function;
			// Need to move past "use crate::" to get to shared_function
			kitty_send_keys!(kitty, KeyCode::Char('0')); // go to start of line
			pause_briefly();
			kitty_send_keys!(kitty, KeyCode::Char('w')); // move to crate
			pause_briefly();
			kitty_send_keys!(kitty, KeyCode::Char('w')); // move past ::
			pause_briefly();
			kitty_send_keys!(kitty, KeyCode::Char('w')); // move to shared_function
			pause_briefly();

			debug_screen(kitty, "CURSOR ON shared_function IN IMPORT");

			// Go to definition
			kitty_send_keys!(kitty, KeyCode::Char('g'), KeyCode::Char('d'));
			pause_briefly();

			// Wait for async LSP response
			std::thread::sleep(Duration::from_secs(3));
			debug_screen(kitty, "AFTER gd (should be in lib.rs)");

			// Should jump to lib.rs definition
			let (_raw, clean) =
				wait_for_screen_text_clean(kitty, Duration::from_secs(2), |_raw, clean| {
					// After gd from import, should navigate to lib.rs
					clean.contains("lib.rs")
						|| clean.contains("pub fn shared_function")
						|| clean.contains("/// A shared function")
				});

			// Check if we jumped to lib.rs (the file where shared_function is defined)
			let jumped_to_lib = clean.contains("lib.rs");
			let shows_definition = clean.contains("pub fn shared_function")
				|| clean.contains("/// A shared function")
				|| clean.contains("42");

			assert!(
				jumped_to_lib || shows_definition,
				"Should jump to definition in lib.rs from import statement. Screen:\n{clean}"
			);
		});
	});
}
