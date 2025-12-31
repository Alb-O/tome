//! Percentage constraint tests.

use rstest::rstest;

use super::letters;
use crate::layout::Constraint::{self, *};
use crate::layout::Flex;

#[rstest]
#[case(Flex::Start, 10, &[Percentage(50)], "aaaaa     ")]
#[case(Flex::Start, 10, &[Percentage(50), Percentage(50)], "aaaaabbbbb")]
fn percentage_constraints(
	#[case] flex: Flex,
	#[case] width: u16,
	#[case] constraints: &[Constraint],
	#[case] expected: &str,
) {
	letters(flex, constraints, width, expected);
}
