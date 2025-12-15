//! Built-in text object registrations.

use linkme::distributed_slice;
use ropey::RopeSlice;

use crate::movement::{select_surround_object, select_word_object, WordType};
use crate::range::Range;

use super::{TextObjectDef, TEXT_OBJECTS};

fn word_inner(text: RopeSlice, pos: usize) -> Option<Range> {
    Some(select_word_object(text, Range::point(pos), WordType::Word, true))
}

fn word_around(text: RopeSlice, pos: usize) -> Option<Range> {
    Some(select_word_object(text, Range::point(pos), WordType::Word, false))
}

#[distributed_slice(TEXT_OBJECTS)]
static OBJ_WORD: TextObjectDef = TextObjectDef {
    name: "word",
    trigger: 'w',
    alt_triggers: &[],
    description: "Select word",
    inner: word_inner,
    around: word_around,
};

fn big_word_inner(text: RopeSlice, pos: usize) -> Option<Range> {
    Some(select_word_object(text, Range::point(pos), WordType::WORD, true))
}

fn big_word_around(text: RopeSlice, pos: usize) -> Option<Range> {
    Some(select_word_object(text, Range::point(pos), WordType::WORD, false))
}

#[distributed_slice(TEXT_OBJECTS)]
static OBJ_WORD_BIG: TextObjectDef = TextObjectDef {
    name: "WORD",
    trigger: 'W',
    alt_triggers: &[],
    description: "Select WORD (non-whitespace)",
    inner: big_word_inner,
    around: big_word_around,
};

fn parens_inner(text: RopeSlice, pos: usize) -> Option<Range> {
    select_surround_object(text, Range::point(pos), '(', ')', true)
}

fn parens_around(text: RopeSlice, pos: usize) -> Option<Range> {
    select_surround_object(text, Range::point(pos), '(', ')', false)
}

#[distributed_slice(TEXT_OBJECTS)]
static OBJ_PARENS: TextObjectDef = TextObjectDef {
    name: "parentheses",
    trigger: 'b',
    alt_triggers: &['(', ')'],
    description: "Select parentheses block",
    inner: parens_inner,
    around: parens_around,
};

fn braces_inner(text: RopeSlice, pos: usize) -> Option<Range> {
    select_surround_object(text, Range::point(pos), '{', '}', true)
}

fn braces_around(text: RopeSlice, pos: usize) -> Option<Range> {
    select_surround_object(text, Range::point(pos), '{', '}', false)
}

#[distributed_slice(TEXT_OBJECTS)]
static OBJ_BRACES: TextObjectDef = TextObjectDef {
    name: "braces",
    trigger: 'B',
    alt_triggers: &['{', '}'],
    description: "Select braces block",
    inner: braces_inner,
    around: braces_around,
};

fn brackets_inner(text: RopeSlice, pos: usize) -> Option<Range> {
    select_surround_object(text, Range::point(pos), '[', ']', true)
}

fn brackets_around(text: RopeSlice, pos: usize) -> Option<Range> {
    select_surround_object(text, Range::point(pos), '[', ']', false)
}

#[distributed_slice(TEXT_OBJECTS)]
static OBJ_BRACKETS: TextObjectDef = TextObjectDef {
    name: "brackets",
    trigger: 'r',
    alt_triggers: &['[', ']'],
    description: "Select brackets block",
    inner: brackets_inner,
    around: brackets_around,
};

fn angle_inner(text: RopeSlice, pos: usize) -> Option<Range> {
    select_surround_object(text, Range::point(pos), '<', '>', true)
}

fn angle_around(text: RopeSlice, pos: usize) -> Option<Range> {
    select_surround_object(text, Range::point(pos), '<', '>', false)
}

#[distributed_slice(TEXT_OBJECTS)]
static OBJ_ANGLE: TextObjectDef = TextObjectDef {
    name: "angle_brackets",
    trigger: 'a',
    alt_triggers: &['<', '>'],
    description: "Select angle brackets block",
    inner: angle_inner,
    around: angle_around,
};

fn double_quotes_inner(text: RopeSlice, pos: usize) -> Option<Range> {
    select_surround_object(text, Range::point(pos), '"', '"', true)
}

fn double_quotes_around(text: RopeSlice, pos: usize) -> Option<Range> {
    select_surround_object(text, Range::point(pos), '"', '"', false)
}

#[distributed_slice(TEXT_OBJECTS)]
static OBJ_DOUBLE_QUOTES: TextObjectDef = TextObjectDef {
    name: "double_quotes",
    trigger: '"',
    alt_triggers: &['Q'],
    description: "Select double-quoted string",
    inner: double_quotes_inner,
    around: double_quotes_around,
};

fn single_quotes_inner(text: RopeSlice, pos: usize) -> Option<Range> {
    select_surround_object(text, Range::point(pos), '\'', '\'', true)
}

fn single_quotes_around(text: RopeSlice, pos: usize) -> Option<Range> {
    select_surround_object(text, Range::point(pos), '\'', '\'', false)
}

#[distributed_slice(TEXT_OBJECTS)]
static OBJ_SINGLE_QUOTES: TextObjectDef = TextObjectDef {
    name: "single_quotes",
    trigger: '\'',
    alt_triggers: &['q'],
    description: "Select single-quoted string",
    inner: single_quotes_inner,
    around: single_quotes_around,
};

fn backticks_inner(text: RopeSlice, pos: usize) -> Option<Range> {
    select_surround_object(text, Range::point(pos), '`', '`', true)
}

fn backticks_around(text: RopeSlice, pos: usize) -> Option<Range> {
    select_surround_object(text, Range::point(pos), '`', '`', false)
}

#[distributed_slice(TEXT_OBJECTS)]
static OBJ_BACKTICKS: TextObjectDef = TextObjectDef {
    name: "backticks",
    trigger: '`',
    alt_triggers: &['g'],
    description: "Select backtick-quoted string",
    inner: backticks_inner,
    around: backticks_around,
};
