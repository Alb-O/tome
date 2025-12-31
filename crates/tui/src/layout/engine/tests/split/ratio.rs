//! Ratio constraint tests.

use rstest::rstest;

use super::letters;
use crate::layout::Constraint::{self, *};
use crate::layout::Flex;

#[rstest]
#[case(Flex::Start, 10, &[Ratio(0, 1), Ratio(0, 1)],   "          " )]
#[case(Flex::Start, 10, &[Ratio(0, 1), Ratio(1, 4)],  "bbb       " )]
#[case(Flex::Start, 10, &[Ratio(0, 1), Ratio(1, 2)],  "bbbbb     " )]
#[case(Flex::Start, 10, &[Ratio(0, 1), Ratio(1, 1)],  "bbbbbbbbbb" )]
#[case(Flex::Start, 10, &[Ratio(0, 1), Ratio(2, 1)],  "bbbbbbbbbb" )]
#[case(Flex::Start, 10, &[Ratio(1, 10), Ratio(0, 1)], "a         " )]
#[case(Flex::Start, 10, &[Ratio(1, 10), Ratio(1, 4)], "abbb      " )]
#[case(Flex::Start, 10, &[Ratio(1, 10), Ratio(1, 2)], "abbbbb    " )]
#[case(Flex::Start, 10, &[Ratio(1, 10), Ratio(1, 1)], "abbbbbbbbb" )]
#[case(Flex::Start, 10, &[Ratio(1, 10), Ratio(2, 1)], "abbbbbbbbb" )]
#[case(Flex::Start, 10, &[Ratio(1, 4), Ratio(0, 1)],  "aaa       " )]
#[case(Flex::Start, 10, &[Ratio(1, 4), Ratio(1, 4)],  "aaabb     " )]
#[case(Flex::Start, 10, &[Ratio(1, 4), Ratio(1, 2)],  "aaabbbbb  " )]
#[case(Flex::Start, 10, &[Ratio(1, 4), Ratio(1, 1)],  "aaabbbbbbb" )]
#[case(Flex::Start, 10, &[Ratio(1, 4), Ratio(2, 1)],  "aaabbbbbbb" )]
#[case(Flex::Start, 10, &[Ratio(1, 3), Ratio(0, 1)],  "aaa       " )]
#[case(Flex::Start, 10, &[Ratio(1, 3), Ratio(1, 4)],  "aaabbb    " )]
#[case(Flex::Start, 10, &[Ratio(1, 3), Ratio(1, 2)],  "aaabbbbb  " )]
#[case(Flex::Start, 10, &[Ratio(1, 3), Ratio(1, 1)],  "aaabbbbbbb" )]
#[case(Flex::Start, 10, &[Ratio(1, 3), Ratio(2, 1)],  "aaabbbbbbb" )]
#[case(Flex::Start, 10, &[Ratio(1, 2), Ratio(0, 1)],  "aaaaa     " )]
#[case(Flex::Start, 10, &[Ratio(1, 2), Ratio(1, 2)],  "aaaaabbbbb" )]
#[case(Flex::Start, 10, &[Ratio(1, 2), Ratio(1, 1)],  "aaaaabbbbb" )]
#[case(Flex::Start, 10, &[Ratio(1, 1), Ratio(0, 1)],  "aaaaaaaaaa" )]
#[case(Flex::Start, 10, &[Ratio(1, 1), Ratio(1, 2)],  "aaaaabbbbb" )]
#[case(Flex::Start, 10, &[Ratio(1, 1), Ratio(1, 1)],  "aaaaabbbbb" )]
#[case(Flex::Start, 10, &[Ratio(1, 1), Ratio(2, 1)],  "aaaaabbbbb" )]
fn ratio_start(
	#[case] flex: Flex,
	#[case] width: u16,
	#[case] constraints: &[Constraint],
	#[case] expected: &str,
) {
	letters(flex, constraints, width, expected);
}

#[rstest]
#[case(Flex::SpaceBetween, 10, &[Ratio(0, 1), Ratio(0, 1)],  "          " )]
#[case(Flex::SpaceBetween, 10, &[Ratio(0, 1), Ratio(1, 4)],  "        bb" )]
#[case(Flex::SpaceBetween, 10, &[Ratio(0, 1), Ratio(1, 2)],  "     bbbbb" )]
#[case(Flex::SpaceBetween, 10, &[Ratio(0, 1), Ratio(1, 1)],  "bbbbbbbbbb" )]
#[case(Flex::SpaceBetween, 10, &[Ratio(0, 1), Ratio(2, 1)],  "bbbbbbbbbb" )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 10), Ratio(0, 1)], "a         " )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 10), Ratio(1, 4)], "a       bb" )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 10), Ratio(1, 2)], "a    bbbbb" )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 10), Ratio(1, 1)], "abbbbbbbbb" )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 10), Ratio(2, 1)], "abbbbbbbbb" )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 4), Ratio(0, 1)],  "aaa       " )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 4), Ratio(1, 4)],  "aaa     bb" )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 4), Ratio(1, 2)],  "aaa  bbbbb" )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 4), Ratio(1, 1)],  "aaabbbbbbb" )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 4), Ratio(2, 1)],  "aaabbbbbbb" )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 3), Ratio(0, 1)],  "aaa       " )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 3), Ratio(1, 4)],  "aaa     bb" )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 3), Ratio(1, 2)],  "aaa  bbbbb" )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 3), Ratio(1, 1)],  "aaabbbbbbb" )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 3), Ratio(2, 1)],  "aaabbbbbbb" )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 2), Ratio(0, 1)],  "aaaaa     " )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 2), Ratio(1, 2)],  "aaaaabbbbb" )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 2), Ratio(1, 1)],  "aaaaabbbbb" )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 1), Ratio(0, 1)],  "aaaaaaaaaa" )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 1), Ratio(1, 2)],  "aaaaabbbbb" )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 1), Ratio(1, 1)],  "aaaaabbbbb" )]
#[case(Flex::SpaceBetween, 10, &[Ratio(1, 1), Ratio(2, 1)],  "aaaaabbbbb" )]
fn ratio_spacebetween(
	#[case] flex: Flex,
	#[case] width: u16,
	#[case] constraints: &[Constraint],
	#[case] expected: &str,
) {
	letters(flex, constraints, width, expected);
}
