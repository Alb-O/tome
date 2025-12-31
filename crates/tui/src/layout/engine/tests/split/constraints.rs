//! Basic constraint tests: Length, Max, Min, and constraint interactions.

use rstest::rstest;

use super::letters;
use crate::layout::Constraint::{self, *};
use crate::layout::Flex;

#[rstest]
#[case(Flex::Start, 5, &[Length(2)], "aa   ")]
#[case(Flex::Start, 5, &[Max(2)], "aa   ")]
#[case(Flex::Start, 5, &[Min(2)], "aaaaa")]
#[case(Flex::Start, 6, &[Length(2), Length(2)], "aabb  ")]
fn basic_constraints(
	#[case] flex: Flex,
	#[case] width: u16,
	#[case] constraints: &[Constraint],
	#[case] expected: &str,
) {
	letters(flex, constraints, width, expected);
}
