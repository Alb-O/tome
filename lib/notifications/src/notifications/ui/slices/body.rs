use ratatui::prelude::*;
use ratatui::widgets::paragraph::Wrap;
use ratatui::widgets::Paragraph;

pub fn render_body(frame: &mut Frame<'_>, area: Rect, content: Text<'static>, style: Style) {
	if area.width == 0 || area.height == 0 {
		return;
	}

	let paragraph = Paragraph::new(content)
		.wrap(Wrap { trim: true })
		.style(style);
	frame.render_widget(paragraph, area);
}
