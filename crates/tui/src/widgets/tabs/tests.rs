use alloc::{format, vec};

use super::*;
use crate::style::{Color, Stylize};
use crate::widgets::borders::BorderType;

#[test]
fn new() {
	let titles = vec!["Tab1", "Tab2", "Tab3", "Tab4"];
	let tabs = Tabs::new(titles.clone());
	assert_eq!(
		tabs,
		Tabs {
			block: None,
			titles: vec![
				Line::from("Tab1"),
				Line::from("Tab2"),
				Line::from("Tab3"),
				Line::from("Tab4"),
			],
			selected: Some(0),
			style: Style::default(),
			highlight_style: DEFAULT_HIGHLIGHT_STYLE,
			divider: Span::raw(symbols::line::VERTICAL),
			padding_right: Line::from(" "),
			padding_left: Line::from(" "),
		}
	);
}

#[test]
fn default() {
	assert_eq!(
		Tabs::default(),
		Tabs {
			block: None,
			titles: vec![],
			selected: None,
			style: Style::default(),
			highlight_style: DEFAULT_HIGHLIGHT_STYLE,
			divider: Span::raw(symbols::line::VERTICAL),
			padding_right: Line::from(" "),
			padding_left: Line::from(" "),
		}
	);
}

#[test]
fn select_into() {
	let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]);
	assert_eq!(tabs.clone().select(2).selected, Some(2));
	assert_eq!(tabs.clone().select(None).selected, None);
	assert_eq!(tabs.clone().select(1u8 as usize).selected, Some(1));
}

#[test]
fn select_before_titles() {
	let tabs = Tabs::default().select(1).titles(["Tab1", "Tab2"]);
	assert_eq!(tabs.selected, Some(1));
}

#[test]
fn new_from_vec_of_str() {
	Tabs::new(vec!["a", "b"]);
}

#[test]
fn collect() {
	let tabs: Tabs = (0..5).map(|i| format!("Tab{i}")).collect();
	assert_eq!(
		tabs.titles,
		vec![
			Line::from("Tab0"),
			Line::from("Tab1"),
			Line::from("Tab2"),
			Line::from("Tab3"),
			Line::from("Tab4"),
		],
	);
}

#[track_caller]
fn test_case(tabs: Tabs, area: Rect, expected: &Buffer) {
	let mut buffer = Buffer::empty(area);
	tabs.render(area, &mut buffer);
	assert_eq!(&buffer, expected);
}

#[test]
fn render_new() {
	let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]);
	let mut expected = Buffer::with_lines([" Tab1 │ Tab2 │ Tab3 │ Tab4    "]);
	// first tab selected
	expected.set_style(Rect::new(1, 0, 4, 1), DEFAULT_HIGHLIGHT_STYLE);
	test_case(tabs, Rect::new(0, 0, 30, 1), &expected);
}

#[test]
fn render_no_padding() {
	let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]).padding("", "");
	let mut expected = Buffer::with_lines(["Tab1│Tab2│Tab3│Tab4           "]);
	// first tab selected
	expected.set_style(Rect::new(0, 0, 4, 1), DEFAULT_HIGHLIGHT_STYLE);
	test_case(tabs, Rect::new(0, 0, 30, 1), &expected);
}

#[test]
fn render_left_padding() {
	let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]).padding_left("---");
	let mut expected = Buffer::with_lines(["---Tab1 │---Tab2 │---Tab3 │---Tab4      "]);
	// first tab selected
	expected.set_style(Rect::new(3, 0, 4, 1), DEFAULT_HIGHLIGHT_STYLE);
	test_case(tabs, Rect::new(0, 0, 40, 1), &expected);
}

#[test]
fn render_right_padding() {
	let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]).padding_right("++");
	let mut expected = Buffer::with_lines([" Tab1++│ Tab2++│ Tab3++│ Tab4++         "]);
	// first tab selected
	expected.set_style(Rect::new(1, 0, 4, 1), DEFAULT_HIGHLIGHT_STYLE);
	test_case(tabs, Rect::new(0, 0, 40, 1), &expected);
}

#[test]
fn render_more_padding() {
	let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]).padding("---", "++");
	let mut expected = Buffer::with_lines(["---Tab1++│---Tab2++│---Tab3++│"]);
	// first tab selected
	expected.set_style(Rect::new(3, 0, 4, 1), DEFAULT_HIGHLIGHT_STYLE);
	test_case(tabs, Rect::new(0, 0, 30, 1), &expected);
}

#[test]
fn render_with_block() {
	let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]).block(
		Block::bordered()
			.border_type(BorderType::Plain)
			.title("Tabs"),
	);
	let mut expected = Buffer::with_lines([
		"┌Tabs────────────────────────┐",
		"│ Tab1 │ Tab2 │ Tab3 │ Tab4  │",
		"└────────────────────────────┘",
	]);
	// first tab selected
	expected.set_style(Rect::new(2, 1, 4, 1), DEFAULT_HIGHLIGHT_STYLE);
	test_case(tabs, Rect::new(0, 0, 30, 3), &expected);
}

#[test]
fn render_style() {
	let tabs =
		Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]).style(Style::default().fg(Color::Red));
	let mut expected = Buffer::with_lines([" Tab1 │ Tab2 │ Tab3 │ Tab4    ".red()]);
	expected.set_style(Rect::new(1, 0, 4, 1), DEFAULT_HIGHLIGHT_STYLE.red());
	test_case(tabs, Rect::new(0, 0, 30, 1), &expected);
}

#[test]
fn render_select() {
	let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]);

	// first tab selected
	let expected = Buffer::with_lines([Line::from(vec![
		" ".into(),
		"Tab1".reversed(),
		" │ Tab2 │ Tab3 │ Tab4    ".into(),
	])]);
	test_case(tabs.clone().select(0), Rect::new(0, 0, 30, 1), &expected);

	// second tab selected
	let expected = Buffer::with_lines([Line::from(vec![
		" Tab1 │ ".into(),
		"Tab2".reversed(),
		" │ Tab3 │ Tab4    ".into(),
	])]);
	test_case(tabs.clone().select(1), Rect::new(0, 0, 30, 1), &expected);

	// last tab selected
	let expected = Buffer::with_lines([Line::from(vec![
		" Tab1 │ Tab2 │ Tab3 │ ".into(),
		"Tab4".reversed(),
		"    ".into(),
	])]);
	test_case(tabs.clone().select(3), Rect::new(0, 0, 30, 1), &expected);

	// out of bounds selects no tab
	let expected = Buffer::with_lines([" Tab1 │ Tab2 │ Tab3 │ Tab4    "]);
	test_case(tabs.clone().select(4), Rect::new(0, 0, 30, 1), &expected);

	// deselect
	let expected = Buffer::with_lines([" Tab1 │ Tab2 │ Tab3 │ Tab4    "]);
	test_case(tabs.clone().select(None), Rect::new(0, 0, 30, 1), &expected);
}

#[test]
fn render_style_and_selected() {
	let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
		.style(Style::new().red())
		.highlight_style(Style::new().underlined())
		.select(0);
	let expected = Buffer::with_lines([Line::from(vec![
		" ".red(),
		"Tab1".red().underlined(),
		" │ Tab2 │ Tab3 │ Tab4    ".red(),
	])]);
	test_case(tabs, Rect::new(0, 0, 30, 1), &expected);
}

#[test]
fn render_divider() {
	let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"]).divider("--");
	let mut expected = Buffer::with_lines([" Tab1 -- Tab2 -- Tab3 -- Tab4 "]);
	// first tab selected
	expected.set_style(Rect::new(1, 0, 4, 1), DEFAULT_HIGHLIGHT_STYLE);
	test_case(tabs, Rect::new(0, 0, 30, 1), &expected);
}

#[test]
fn can_be_stylized() {
	assert_eq!(
		Tabs::new(vec![""])
			.black()
			.on_white()
			.bold()
			.not_italic()
			.style,
		Style::default().black().on_white().bold().not_italic()
	);
}

#[test]
fn render_in_minimal_buffer() {
	let mut buffer = Buffer::empty(Rect::new(0, 0, 1, 1));
	let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
		.select(1)
		.divider("|");
	// This should not panic, even if the buffer is too small to render the tabs.
	tabs.render(buffer.area, &mut buffer);
	assert_eq!(buffer, Buffer::with_lines([" "]));
}

#[test]
fn render_in_zero_size_buffer() {
	let mut buffer = Buffer::empty(Rect::ZERO);
	let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3", "Tab4"])
		.select(1)
		.divider("|");
	// This should not panic, even if the buffer has zero size.
	tabs.render(buffer.area, &mut buffer);
}

#[test]
fn unicode_width_basic() {
	let tabs = Tabs::new(vec!["A", "BB", "CCC"]);
	let rendered = " A │ BB │ CCC ";
	assert_eq!(tabs.width(), rendered.width());
}

#[test]
fn unicode_width_no_padding() {
	let tabs = Tabs::new(vec!["A", "BB", "CCC"]).padding("", "");
	let rendered = "A│BB│CCC";
	assert_eq!(tabs.width(), rendered.width());
}

#[test]
fn unicode_width_custom_divider_and_padding() {
	let tabs = Tabs::new(vec!["A", "BB", "CCC"])
		.divider("--")
		.padding("X", "YY");
	let rendered = "XAYY--XBBYY--XCCCYY";
	assert_eq!(tabs.width(), rendered.width());
}

#[test]
fn unicode_width_empty_titles() {
	let tabs = Tabs::new(Vec::<&str>::new());
	let rendered = "";
	assert_eq!(tabs.width(), rendered.width());
}

#[test]
fn unicode_width_cjk() {
	let tabs = Tabs::new(vec!["你", "好", "世界"]);
	let rendered = " 你 │ 好 │ 世界 ";
	assert_eq!(tabs.width_cjk(), UnicodeWidthStr::width_cjk(rendered));
}

#[test]
fn unicode_width_cjk_custom_padding_and_divider() {
	let tabs = Tabs::new(vec!["你", "好", "世界"])
		.divider("分")
		.padding("左", "右");
	let rendered = "左你右分左好右分左世界右";
	assert_eq!(tabs.width_cjk(), UnicodeWidthStr::width_cjk(rendered));
}
