use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

use crate::notifications::ui::chrome::GutterLayout;

pub fn render_icon_gutter(
	frame: &mut Frame<'_>,
	area: Rect,
	gutter: GutterLayout,
	icon: &str,
	style: Style,
) {
	if area.width == 0 || area.height == 0 {
		return;
	}

	// Vertically center the icon within the gutter column.
	let icon_v_chunks = Layout::default()
		.direction(Direction::Vertical)
		.constraints([
			Constraint::Fill(1),
			Constraint::Length(1),
			Constraint::Fill(1),
		])
		.split(area);

	let mut icon_area = icon_v_chunks[1];
	icon_area.x = icon_area.x.saturating_add(gutter.left_pad);
	icon_area.width = icon_area.width.saturating_sub(gutter.left_pad);

	let paragraph = Paragraph::new(icon).style(style).alignment(Alignment::Left);
	frame.render_widget(paragraph, icon_area);
}
