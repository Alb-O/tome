//! Fill constraint tests.

use alloc::vec;
use alloc::vec::Vec;
use core::ops::Range;

use itertools::Itertools;
use pretty_assertions::assert_eq;
use rstest::rstest;

use crate::layout::Constraint::{self, *};
use crate::layout::{Flex, Layout, Rect};

#[rstest]
#[case::fill_proportional(vec![Fill(1), Fill(2), Fill(1)], vec![0..25, 25..75, 75..100])]
#[case::fill_with_fixed(vec![Length(10), Fill(1), Length(10)], vec![0..10, 10..90, 90..100])]
fn fill_distribution(#[case] constraints: Vec<Constraint>, #[case] expected: Vec<Range<u16>>) {
	let rect = Rect::new(0, 0, 100, 1);
	let ranges = Layout::horizontal(constraints)
		.flex(Flex::Start)
		.split(rect)
		.iter()
		.map(|r| r.left()..r.right())
		.collect_vec();
	assert_eq!(ranges, expected);
}

#[rstest]
#[case::fill_spacing(vec![(0, 45), (55, 45)], vec![Fill(1), Fill(1)], 10)]
#[case::mixed_spacing(vec![(0, 10), (20, 60), (90, 10)], vec![Length(10), Fill(1), Length(10)], 10)]
fn fill_with_spacing(
	#[case] expected: Vec<(u16, u16)>,
	#[case] constraints: Vec<Constraint>,
	#[case] spacing: i16,
) {
	let rect = Rect::new(0, 0, 100, 1);
	let result = Layout::horizontal(constraints)
		.flex(Flex::Start)
		.spacing(spacing)
		.split(rect)
		.iter()
		.map(|r| (r.x, r.width))
		.collect::<Vec<(u16, u16)>>();
	assert_eq!(result, expected);
}
