use crate::graphemes::{next_grapheme_boundary, prev_grapheme_boundary};
use crate::range::{Direction, Range};
use ropey::RopeSlice;

pub fn move_horizontally(
    text: RopeSlice,
    range: Range,
    direction: Direction,
    count: usize,
    extend: bool,
) -> Range {
    let pos = range.head;
    let new_pos = match direction {
        Direction::Forward => {
            let mut p = pos;
            for _ in 0..count {
                p = next_grapheme_boundary(text, p);
            }
            p
        }
        Direction::Backward => {
            let mut p = pos;
            for _ in 0..count {
                p = prev_grapheme_boundary(text, p);
            }
            p
        }
    };

    if extend {
        Range::new(range.anchor, new_pos)
    } else {
        Range::point(new_pos)
    }
}

pub fn move_vertically(
    text: RopeSlice,
    range: Range,
    direction: Direction,
    count: usize,
    extend: bool,
) -> Range {
    let pos = range.head;
    let line = text.char_to_line(pos);
    let line_start = text.line_to_char(line);
    let col = pos - line_start;

    let new_line = match direction {
        Direction::Forward => (line + count).min(text.len_lines().saturating_sub(1)),
        Direction::Backward => line.saturating_sub(count),
    };

    let new_line_start = text.line_to_char(new_line);
    let new_line_len = text.line(new_line).len_chars();
    let line_end_offset = if new_line == text.len_lines().saturating_sub(1) {
        new_line_len
    } else {
        new_line_len.saturating_sub(1)
    };

    let new_col = col.min(line_end_offset);
    let new_pos = new_line_start + new_col;

    if extend {
        Range::new(range.anchor, new_pos)
    } else {
        Range::point(new_pos)
    }
}

pub fn move_to_line_start(text: RopeSlice, range: Range, extend: bool) -> Range {
    let line = text.char_to_line(range.head);
    let line_start = text.line_to_char(line);

    if extend {
        Range::new(range.anchor, line_start)
    } else {
        Range::point(line_start)
    }
}

pub fn move_to_line_end(text: RopeSlice, range: Range, extend: bool) -> Range {
    let line = text.char_to_line(range.head);
    let line_start = text.line_to_char(line);
    let line_len = text.line(line).len_chars();

    let is_last_line = line == text.len_lines().saturating_sub(1);
    let line_end = if is_last_line {
        line_start + line_len
    } else {
        line_start + line_len.saturating_sub(1)
    };

    if extend {
        Range::new(range.anchor, line_end)
    } else {
        Range::point(line_end)
    }
}

pub fn move_to_first_nonwhitespace(text: RopeSlice, range: Range, extend: bool) -> Range {
    let line = text.char_to_line(range.head);
    let line_start = text.line_to_char(line);
    let line_text = text.line(line);

    let mut first_non_ws = line_start;
    for (i, ch) in line_text.chars().enumerate() {
        if !ch.is_whitespace() {
            first_non_ws = line_start + i;
            break;
        }
    }

    if extend {
        Range::new(range.anchor, first_non_ws)
    } else {
        Range::point(first_non_ws)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ropey::Rope;

    #[test]
    fn test_move_forward() {
        let text = Rope::from("hello world");
        let slice = text.slice(..);
        let range = Range::point(0);

        let moved = move_horizontally(slice, range, Direction::Forward, 1, false);
        assert_eq!(moved.head, 1);
    }

    #[test]
    fn test_move_backward() {
        let text = Rope::from("hello world");
        let slice = text.slice(..);
        let range = Range::point(5);

        let moved = move_horizontally(slice, range, Direction::Backward, 2, false);
        assert_eq!(moved.head, 3);
    }

    #[test]
    fn test_move_forward_extend() {
        let text = Rope::from("hello world");
        let slice = text.slice(..);
        let range = Range::point(0);

        let moved = move_horizontally(slice, range, Direction::Forward, 5, true);
        assert_eq!(moved.anchor, 0);
        assert_eq!(moved.head, 5);
    }

    #[test]
    fn test_move_down() {
        let text = Rope::from("hello\nworld\n");
        let slice = text.slice(..);
        let range = Range::point(2);

        let moved = move_vertically(slice, range, Direction::Forward, 1, false);
        assert_eq!(moved.head, 8);
    }

    #[test]
    fn test_move_up() {
        let text = Rope::from("hello\nworld\n");
        let slice = text.slice(..);
        let range = Range::point(8);

        let moved = move_vertically(slice, range, Direction::Backward, 1, false);
        assert_eq!(moved.head, 2);
    }

    #[test]
    fn test_move_to_line_start() {
        let text = Rope::from("hello\nworld\n");
        let slice = text.slice(..);
        let range = Range::point(8);

        let moved = move_to_line_start(slice, range, false);
        assert_eq!(moved.head, 6);
    }

    #[test]
    fn test_move_to_line_end() {
        let text = Rope::from("hello\nworld\n");
        let slice = text.slice(..);
        let range = Range::point(6);

        let moved = move_to_line_end(slice, range, false);
        assert_eq!(moved.head, 11);
    }

    #[test]
    fn test_move_to_first_nonwhitespace() {
        let text = Rope::from("  hello\n");
        let slice = text.slice(..);
        let range = Range::point(0);

        let moved = move_to_first_nonwhitespace(slice, range, false);
        assert_eq!(moved.head, 2);
    }
}
