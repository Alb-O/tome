use ratatui::layout::{Alignment, Rect};
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

	// Prefer a consistent top-aligned icon (avoids visual "jumps" when content wraps).
	let mut icon_area = Rect {
		x: area.x,
		y: area.y,
		width: area.width,
		height: 1,
	};
	icon_area.x = icon_area.x.saturating_add(gutter.left_pad);
	icon_area.width = icon_area.width.saturating_sub(gutter.left_pad);

	let paragraph = Paragraph::new(icon).style(style).alignment(Alignment::Left);
	frame.render_widget(paragraph, icon_area);
}
