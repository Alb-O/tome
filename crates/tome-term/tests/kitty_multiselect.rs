use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

use kitty_test_harness::{kitty_send_keys, with_kitty_capture, wait_for_screen_text_clean, pause_briefly, run_with_timeout};
use termwiz::input::KeyCode;

const TEST_TIMEOUT: Duration = Duration::from_secs(15);

fn tome_cmd() -> String {
    env!("CARGO_BIN_EXE_tome").to_string()
}

fn tome_cmd_with_file() -> String {
    format!("{} {}", tome_cmd(), "kitty-test.txt")
}

fn workspace_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn require_kitty() -> bool {
    let wants_kitty = std::env::var("KITTY_TESTS").unwrap_or_default();
    if wants_kitty.is_empty() || wants_kitty == "0" || wants_kitty.eq_ignore_ascii_case("false") {
        eprintln!("skipping kitty tests: set KITTY_TESTS=1 and run under a GUI session");
        return false;
    }

    let has_display = std::env::var("DISPLAY").is_ok() || std::env::var("WAYLAND_DISPLAY").is_ok();
    if !has_display {
        eprintln!("skipping kitty tests: DISPLAY/WAYLAND_DISPLAY not set");
        return false;
    }

    let kitty_ok = Command::new("kitty").arg("--version").output().is_ok();
    if !kitty_ok {
        eprintln!("skipping kitty tests: kitty binary not found on PATH");
    }
    kitty_ok
}
#[test]
fn harness_can_insert_and_capture() {
    if !require_kitty() {
        return;
    }

    run_with_timeout(TEST_TIMEOUT, || {
        with_kitty_capture(&workspace_dir(), &tome_cmd_with_file(), |kitty| {
            pause_briefly();

            kitty_send_keys!(kitty, KeyCode::Char('i'));
            kitty.send_text("hello kitty harness\n");
            kitty_send_keys!(kitty, KeyCode::Escape);

            let (_raw, clean) = wait_for_screen_text_clean(kitty, Duration::from_secs(3), |_r, clean| {
                clean.contains("hello kitty harness")
            });

            assert!(clean.contains("hello kitty harness"));
        });
    });
}

#[test]
fn harness_macro_keys_handle_newlines() {
    if !require_kitty() {
        return;
    }

    run_with_timeout(TEST_TIMEOUT, || {
        with_kitty_capture(&workspace_dir(), &tome_cmd_with_file(), |kitty| {
            pause_briefly();

            kitty_send_keys!(kitty, KeyCode::Char('i'));
            kitty_send_keys!(kitty, KeyCode::Char('A'), KeyCode::Char('B'), KeyCode::Enter, KeyCode::Char('C'));
            kitty_send_keys!(kitty, KeyCode::Escape);

            let (_raw, clean) = wait_for_screen_text_clean(kitty, Duration::from_secs(3), |_r, clean| {
                clean.contains("AB") && clean.contains("C")
            });

            assert!(clean.contains("AB"));
            assert!(clean.contains("C"));
        });
    });
}

#[test]
fn split_lines_adds_multi_selection_highlights() {
    if !require_kitty() {
        return;
    }

    run_with_timeout(TEST_TIMEOUT, || {
        with_kitty_capture(&workspace_dir(), &tome_cmd_with_file(), |kitty| {
            pause_briefly();

            // Populate a small buffer; direct text send is more reliable for setup.
            kitty_send_keys!(kitty, KeyCode::Char('i'));
            kitty.send_text("one\ntwo\nthree\n");
            kitty_send_keys!(kitty, KeyCode::Escape);
            pause_briefly();

            // Ensure the text actually landed before proceeding.
            let (_raw, clean_initial) = wait_for_screen_text_clean(kitty, Duration::from_secs(3), |_r, clean| {
                clean.contains("three")
            });
            assert!(clean_initial.contains("one"));

            // Select everything then split into per-line selections (Alt-s).
            kitty_send_keys!(kitty, KeyCode::Char('%'));
            kitty_send_keys!(kitty, (KeyCode::Char('s'), termwiz::input::Modifiers::ALT));

            let (raw, clean) = wait_for_screen_text_clean(kitty, Duration::from_secs(3), |_raw, clean| {
                clean.contains("three")
            });

            // Expect multiple background color spans for selections (kitty extended SGR uses colons).
            let selection_hits = raw.matches("48:2:").count();
            assert!(selection_hits >= 3, "expected selection highlight across lines, saw {selection_hits}, raw: {raw:?}");

            assert!(clean.contains("one"));
            assert!(clean.contains("two"));
            assert!(clean.contains("three"));
        });
    });
}

#[test]
fn duplicate_down_then_delete_removes_adjacent_line() {
    if !require_kitty() {
        return;
    }

    run_with_timeout(TEST_TIMEOUT, || {
        with_kitty_capture(&workspace_dir(), &tome_cmd_with_file(), |kitty| {
            pause_briefly();

            kitty_send_keys!(kitty,
                KeyCode::Char('i'),
                KeyCode::Char('a'), KeyCode::Char('l'), KeyCode::Char('p'), KeyCode::Char('h'), KeyCode::Char('a'), KeyCode::Enter,
                KeyCode::Char('b'), KeyCode::Char('e'), KeyCode::Char('t'), KeyCode::Char('a'), KeyCode::Enter,
                KeyCode::Char('g'), KeyCode::Char('a'), KeyCode::Char('m'), KeyCode::Char('m'), KeyCode::Char('a'), KeyCode::Enter,
            );
            kitty_send_keys!(kitty, KeyCode::Escape);
            pause_briefly();

            // Confirm text is present before manipulations.
            let (_raw, clean_initial) = wait_for_screen_text_clean(kitty, Duration::from_secs(3), |_r, clean| {
                clean.contains("gamma")
            });
            assert!(clean_initial.contains("alpha"));

            // Move to the second line and select it.
            kitty_send_keys!(kitty, KeyCode::Char('g'), KeyCode::Char('g'));
            kitty_send_keys!(kitty, KeyCode::Char('j'));
            kitty_send_keys!(kitty, KeyCode::Char('0'));
            kitty_send_keys!(kitty, KeyCode::Char('x'));
            pause_briefly();

            // Duplicate the selection down onto the next line, then delete both selections.
            kitty_send_keys!(kitty, KeyCode::Char('+'));
            kitty_send_keys!(kitty, KeyCode::Char('d'));


            let (_raw, clean) = wait_for_screen_text_clean(kitty, Duration::from_secs(3), |_raw, clean| {
                clean.contains("alpha")
            });

            assert!(clean.contains("alpha"), "buffer after delete: {clean:?}");
            assert!(!clean.contains("beta"), "buffer after delete: {clean:?}");
            assert!(!clean.contains("gamma"), "buffer after delete: {clean:?}");
        });
    });
}


