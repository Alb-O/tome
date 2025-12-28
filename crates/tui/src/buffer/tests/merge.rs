//! Tests for Buffer merge functionality.

use rstest::rstest;

use super::*;

#[rstest]
#[case(Rect::new(0, 0, 2, 2), Rect::new(0, 2, 2, 2), ["11", "11", "22", "22"])]
#[case(Rect::new(2, 2, 2, 2), Rect::new(0, 0, 2, 2), ["22  ", "22  ", "  11", "  11"])]
fn merge<'line, Lines>(#[case] one: Rect, #[case] two: Rect, #[case] expected: Lines)
where
	Lines: IntoIterator,
	Lines::Item: Into<Line<'line>>,
{
	let mut one = Buffer::filled(one, Cell::new("1"));
	let two = Buffer::filled(two, Cell::new("2"));
	one.merge(&two);
	assert_eq!(one, Buffer::with_lines(expected));
}

#[test]
fn merge_with_offset() {
	let mut one = Buffer::filled(
		Rect {
			x: 3,
			y: 3,
			width: 2,
			height: 2,
		},
		Cell::new("1"),
	);
	let two = Buffer::filled(
		Rect {
			x: 1,
			y: 1,
			width: 3,
			height: 4,
		},
		Cell::new("2"),
	);
	one.merge(&two);
	let mut expected = Buffer::with_lines(["222 ", "222 ", "2221", "2221"]);
	expected.area = Rect {
		x: 1,
		y: 1,
		width: 4,
		height: 4,
	};
	assert_eq!(one, expected);
}

#[rstest]
#[case(false, true, [false, false, true, true, true, true])]
#[case(true, false, [true, true, false, false, false, false])]
fn merge_skip(#[case] skip_one: bool, #[case] skip_two: bool, #[case] expected: [bool; 6]) {
	let mut one = {
		let area = Rect {
			x: 0,
			y: 0,
			width: 2,
			height: 2,
		};
		let mut cell = Cell::new("1");
		cell.skip = skip_one;
		Buffer::filled(area, cell)
	};
	let two = {
		let area = Rect {
			x: 0,
			y: 1,
			width: 2,
			height: 2,
		};
		let mut cell = Cell::new("2");
		cell.skip = skip_two;
		Buffer::filled(area, cell)
	};
	one.merge(&two);
	let skipped = one.content().iter().map(|c| c.skip).collect::<Vec<_>>();
	assert_eq!(skipped, expected);
}
