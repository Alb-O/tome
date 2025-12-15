//! Built-in motion registrations.

use linkme::distributed_slice;
use ropey::RopeSlice;

use crate::movement::{
    self, move_horizontally, move_to_document_end, move_to_document_start,
    move_to_first_nonwhitespace, move_to_line_end, move_to_line_start,
    move_to_next_word_end, move_to_next_word_start, move_to_prev_word_start,
    move_vertically, WordType,
};
use crate::range::{Direction, Range};

use super::{MotionDef, MOTIONS};

// === Basic movement ===

fn move_left(text: RopeSlice, range: Range, count: usize, extend: bool) -> Range {
    move_horizontally(text, range, Direction::Backward, count, extend)
}

#[distributed_slice(MOTIONS)]
static MOTION_LEFT: MotionDef = MotionDef {
    name: "move_left",
    description: "Move left",
    handler: move_left,
};

fn move_right(text: RopeSlice, range: Range, count: usize, extend: bool) -> Range {
    move_horizontally(text, range, Direction::Forward, count, extend)
}

#[distributed_slice(MOTIONS)]
static MOTION_RIGHT: MotionDef = MotionDef {
    name: "move_right",
    description: "Move right",
    handler: move_right,
};

fn move_up(text: RopeSlice, range: Range, count: usize, extend: bool) -> Range {
    move_vertically(text, range, Direction::Backward, count, extend)
}

#[distributed_slice(MOTIONS)]
static MOTION_UP: MotionDef = MotionDef {
    name: "move_up",
    description: "Move up",
    handler: move_up,
};

fn move_down(text: RopeSlice, range: Range, count: usize, extend: bool) -> Range {
    move_vertically(text, range, Direction::Forward, count, extend)
}

#[distributed_slice(MOTIONS)]
static MOTION_DOWN: MotionDef = MotionDef {
    name: "move_down",
    description: "Move down",
    handler: move_down,
};

// === Word movement ===

fn next_word_start(text: RopeSlice, range: Range, count: usize, extend: bool) -> Range {
    move_to_next_word_start(text, range, count, WordType::Word, extend)
}

#[distributed_slice(MOTIONS)]
static MOTION_NEXT_WORD_START: MotionDef = MotionDef {
    name: "next_word_start",
    description: "Move to next word start",
    handler: next_word_start,
};

fn prev_word_start(text: RopeSlice, range: Range, count: usize, extend: bool) -> Range {
    move_to_prev_word_start(text, range, count, WordType::Word, extend)
}

#[distributed_slice(MOTIONS)]
static MOTION_PREV_WORD_START: MotionDef = MotionDef {
    name: "prev_word_start",
    description: "Move to previous word start",
    handler: prev_word_start,
};

fn next_word_end(text: RopeSlice, range: Range, count: usize, extend: bool) -> Range {
    move_to_next_word_end(text, range, count, WordType::Word, extend)
}

#[distributed_slice(MOTIONS)]
static MOTION_NEXT_WORD_END: MotionDef = MotionDef {
    name: "next_word_end",
    description: "Move to next word end",
    handler: next_word_end,
};

fn next_big_word_start(text: RopeSlice, range: Range, count: usize, extend: bool) -> Range {
    move_to_next_word_start(text, range, count, WordType::WORD, extend)
}

#[distributed_slice(MOTIONS)]
static MOTION_NEXT_BIG_WORD_START: MotionDef = MotionDef {
    name: "next_WORD_start",
    description: "Move to next WORD start",
    handler: next_big_word_start,
};

fn prev_big_word_start(text: RopeSlice, range: Range, count: usize, extend: bool) -> Range {
    move_to_prev_word_start(text, range, count, WordType::WORD, extend)
}

#[distributed_slice(MOTIONS)]
static MOTION_PREV_BIG_WORD_START: MotionDef = MotionDef {
    name: "prev_WORD_start",
    description: "Move to previous WORD start",
    handler: prev_big_word_start,
};

fn next_big_word_end(text: RopeSlice, range: Range, count: usize, extend: bool) -> Range {
    move_to_next_word_end(text, range, count, WordType::WORD, extend)
}

#[distributed_slice(MOTIONS)]
static MOTION_NEXT_BIG_WORD_END: MotionDef = MotionDef {
    name: "next_WORD_end",
    description: "Move to next WORD end",
    handler: next_big_word_end,
};

// === Line movement ===

fn line_start(text: RopeSlice, range: Range, _count: usize, extend: bool) -> Range {
    move_to_line_start(text, range, extend)
}

#[distributed_slice(MOTIONS)]
static MOTION_LINE_START: MotionDef = MotionDef {
    name: "line_start",
    description: "Move to line start",
    handler: line_start,
};

fn line_end(text: RopeSlice, range: Range, _count: usize, extend: bool) -> Range {
    move_to_line_end(text, range, extend)
}

#[distributed_slice(MOTIONS)]
static MOTION_LINE_END: MotionDef = MotionDef {
    name: "line_end",
    description: "Move to line end",
    handler: line_end,
};

fn first_nonwhitespace(text: RopeSlice, range: Range, _count: usize, extend: bool) -> Range {
    move_to_first_nonwhitespace(text, range, extend)
}

#[distributed_slice(MOTIONS)]
static MOTION_FIRST_NONWHITESPACE: MotionDef = MotionDef {
    name: "first_nonwhitespace",
    description: "Move to first non-whitespace",
    handler: first_nonwhitespace,
};

// === Document movement ===

fn document_start(text: RopeSlice, range: Range, _count: usize, extend: bool) -> Range {
    move_to_document_start(text, range, extend)
}

#[distributed_slice(MOTIONS)]
static MOTION_DOCUMENT_START: MotionDef = MotionDef {
    name: "document_start",
    description: "Move to document start",
    handler: document_start,
};

fn document_end(text: RopeSlice, range: Range, _count: usize, extend: bool) -> Range {
    move_to_document_end(text, range, extend)
}

#[distributed_slice(MOTIONS)]
static MOTION_DOCUMENT_END: MotionDef = MotionDef {
    name: "document_end",
    description: "Move to document end",
    handler: document_end,
};

// === Find character ===

fn find_char_forward(_text: RopeSlice, range: Range, _count: usize, extend: bool) -> Range {
    // Note: This is a placeholder - the actual char is passed differently
    // The find_char motion requires the character to search for, which is
    // handled specially by the input system
    movement::make_range(range.anchor, range.head, extend)
}

#[distributed_slice(MOTIONS)]
static MOTION_FIND_CHAR_FORWARD: MotionDef = MotionDef {
    name: "find_char_forward",
    description: "Find character forward (placeholder)",
    handler: find_char_forward,
};
