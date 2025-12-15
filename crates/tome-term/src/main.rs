mod commands;
mod editor;
mod render;

use std::env;
use std::io;
use std::path::PathBuf;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture, Event};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

pub use editor::Editor;

fn run_editor(mut editor: Editor) -> io::Result<()> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = (|| {
        loop {
            let viewport_height = terminal.size()?.height.saturating_sub(2) as usize;
            editor.adjust_scroll(viewport_height);

            terminal.draw(|frame| editor.render(frame))?;

            if let Event::Key(key) = crossterm::event::read()? {
                if key.kind == crossterm::event::KeyEventKind::Press {
                    if editor.handle_key(key) {
                        break;
                    }
                }
            }
        }
        Ok(())
    })();

    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    result
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: tome <file>");
        std::process::exit(1);
    }

    let path = PathBuf::from(&args[1]);
    let editor = Editor::new(path)?;
    run_editor(editor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use insta::assert_snapshot;
    use ratatui::{Terminal, backend::TestBackend};
    use tome_core::Mode;

    fn test_editor(content: &str) -> Editor {
        Editor::from_content(content.to_string(), PathBuf::from("test.txt"))
    }

    #[test]
    fn test_render_empty() {
        let editor = test_editor("");
        let mut terminal = Terminal::new(TestBackend::new(80, 10)).unwrap();
        terminal.draw(|frame| editor.render(frame)).unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_with_content() {
        let editor = test_editor("Hello, World!\nThis is a test.\nLine 3.");
        let mut terminal = Terminal::new(TestBackend::new(80, 10)).unwrap();
        terminal.draw(|frame| editor.render(frame)).unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_insert_mode() {
        let mut editor = test_editor("Hello");
        editor.input.set_mode(Mode::Insert);
        let mut terminal = Terminal::new(TestBackend::new(80, 10)).unwrap();
        terminal.draw(|frame| editor.render(frame)).unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_after_typing() {
        let mut editor = test_editor("");
        editor.input.set_mode(Mode::Insert);
        editor.insert_text("abc");
        let mut terminal = Terminal::new(TestBackend::new(80, 10)).unwrap();
        terminal.draw(|frame| editor.render(frame)).unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_with_selection() {
        let mut editor = test_editor("Hello, World!");
        editor.handle_key(KeyEvent::new(KeyCode::Char('L'), KeyModifiers::SHIFT));
        editor.handle_key(KeyEvent::new(KeyCode::Char('L'), KeyModifiers::SHIFT));
        editor.handle_key(KeyEvent::new(KeyCode::Char('L'), KeyModifiers::SHIFT));
        let mut terminal = Terminal::new(TestBackend::new(80, 10)).unwrap();
        terminal.draw(|frame| editor.render(frame)).unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_render_cursor_movement() {
        let mut editor = test_editor("Hello\nWorld");
        editor.handle_key(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE));
        editor.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
        editor.handle_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
        let mut terminal = Terminal::new(TestBackend::new(80, 10)).unwrap();
        terminal.draw(|frame| editor.render(frame)).unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_word_movement() {
        let mut editor = test_editor("hello world test");
        editor.handle_key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::NONE));
        assert_eq!(editor.selection.primary().head, 6);
    }

    #[test]
    fn test_goto_mode() {
        let mut editor = test_editor("line1\nline2\nline3");
        editor.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
        assert!(matches!(editor.mode(), Mode::Goto));
        editor.handle_key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::NONE));
        assert_eq!(editor.selection.primary().head, 0);
    }

    #[test]
    fn test_undo_redo() {
        let mut editor = test_editor("hello");
        assert_eq!(editor.doc.to_string(), "hello");

        editor.handle_key(KeyEvent::new(KeyCode::Char('%'), KeyModifiers::NONE));
        assert_eq!(editor.selection.primary().from(), 0);
        assert_eq!(editor.selection.primary().to(), 5);

        editor.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));
        assert_eq!(editor.doc.to_string(), "", "after delete");
        assert_eq!(editor.undo_stack.len(), 1, "undo stack should have 1 entry");

        editor.handle_key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::NONE));
        assert_eq!(editor.doc.to_string(), "hello", "after undo");
        assert_eq!(editor.redo_stack.len(), 1, "redo stack should have 1 entry");
        assert_eq!(editor.undo_stack.len(), 0, "undo stack should be empty");

        editor.handle_key(KeyEvent::new(KeyCode::Char('U'), KeyModifiers::SHIFT));
        assert_eq!(editor.redo_stack.len(), 0, "redo stack should be empty after redo");
        assert_eq!(editor.doc.to_string(), "", "after redo");
    }

    #[test]
    fn test_insert_newline_single_cursor() {
        use ratatui::style::{Color, Modifier};
        
        let mut editor = test_editor("");
        
        editor.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
        assert!(matches!(editor.mode(), Mode::Insert));
        
        editor.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        
        assert_eq!(editor.doc.len_lines(), 2, "should have 2 lines after Enter");
        assert_eq!(editor.selection.primary().head, 1, "cursor should be at position 1");
        
        let mut terminal = Terminal::new(TestBackend::new(80, 10)).unwrap();
        terminal.draw(|frame| editor.render(frame)).unwrap();
        
        let buffer = terminal.backend().buffer();
        let mut cursor_cells = Vec::new();
        for row in 0..8 {
            for col in 0..80 {
                let cell = &buffer[(col, row)];
                if cell.bg == Color::White && cell.fg == Color::Black 
                   && cell.modifier.contains(Modifier::BOLD) {
                    cursor_cells.push((col, row));
                }
            }
        }
        
        assert_eq!(
            cursor_cells.len(), 
            1, 
            "Expected 1 cursor cell, found {} at positions: {:?}", 
            cursor_cells.len(), 
            cursor_cells
        );
        assert_eq!(
            cursor_cells[0].1, 
            1, 
            "Cursor should be on row 1 (second line), found at {:?}", 
            cursor_cells[0]
        );
        
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_insert_mode_arrow_keys() {
        let mut editor = test_editor("hello world");
        assert_eq!(editor.selection.primary().head, 0, "start at position 0");

        editor.handle_key(KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
        assert!(matches!(editor.mode(), Mode::Insert), "should be in insert mode");

        editor.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
        assert_eq!(editor.selection.primary().head, 1, "after Right arrow, cursor at 1");

        editor.handle_key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
        assert_eq!(editor.selection.primary().head, 2, "after Right arrow, cursor at 2");

        editor.handle_key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
        assert_eq!(editor.selection.primary().head, 1, "after Left arrow, cursor at 1");

        editor.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        assert!(matches!(editor.mode(), Mode::Insert), "still in insert mode after arrows");
    }

    #[test]
    fn test_soft_wrap_long_line() {
        let long_line = "The quick brown fox jumps over the lazy dog and keeps on running";
        let editor = test_editor(long_line);
        
        let mut terminal = Terminal::new(TestBackend::new(40, 10)).unwrap();
        terminal.draw(|frame| editor.render(frame)).unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_soft_wrap_word_boundary() {
        let text = "hello world this is a test of word wrapping behavior";
        let editor = test_editor(text);
        
        let mut terminal = Terminal::new(TestBackend::new(30, 10)).unwrap();
        terminal.draw(|frame| editor.render(frame)).unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_line_numbers_multiple_lines() {
        let text = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
        let editor = test_editor(text);
        
        let mut terminal = Terminal::new(TestBackend::new(40, 10)).unwrap();
        terminal.draw(|frame| editor.render(frame)).unwrap();
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_wrapped_line_dim_gutter() {
        use ratatui::style::Color;
        
        let long_line = "This is a very long line that should wrap to multiple virtual lines";
        let editor = test_editor(long_line);
        
        let mut terminal = Terminal::new(TestBackend::new(30, 10)).unwrap();
        terminal.draw(|frame| editor.render(frame)).unwrap();
        
        let buffer = terminal.backend().buffer();
        
        let first_gutter = &buffer[(0, 0)];
        assert_eq!(first_gutter.fg, Color::DarkGray, "first line gutter should be DarkGray");
        
        let second_gutter = &buffer[(0, 1)];
        assert_eq!(second_gutter.fg, Color::Rgb(60, 60, 60), "wrapped line gutter should be dim");
        
        assert_snapshot!(terminal.backend());
    }

    #[test]
    fn test_backspace_deletes_backwards() {
        let mut editor = test_editor("hello");
        
        editor.handle_key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
        assert!(matches!(editor.mode(), Mode::Insert));
        assert_eq!(editor.selection.primary().head, 1, "cursor at 1 after 'a'");
        
        editor.handle_key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
        assert_eq!(editor.doc.to_string(), "ello", "first char deleted");
        assert_eq!(editor.selection.primary().head, 0, "cursor moved back to 0");
        
        editor.handle_key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
        assert_eq!(editor.doc.to_string(), "ello", "no change when at start");
        assert_eq!(editor.selection.primary().head, 0, "cursor stays at 0");
    }
}
