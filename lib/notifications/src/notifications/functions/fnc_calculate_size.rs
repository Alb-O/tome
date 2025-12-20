use ratatui::buffer::Buffer;
use ratatui::prelude::*;
use ratatui::widgets::paragraph::Wrap;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

use crate::notifications::classes::Notification;
use crate::notifications::types::SizeConstraint;
use crate::notifications::ui::chrome::{gutter_layout, padding_with_gutter};

/// Calculates the size of a notification based on its content and constraints.
///
/// This function determines the width and height needed to display a notification,
/// taking into account borders, padding, gutter/icon column, content wrapping, and
/// size constraints.
///
/// Important: this measurement must match how rendering splits out the icon gutter.
/// We model the gutter as additional left padding during measurement.
pub fn calculate_size(notification: &Notification, frame_area: Rect) -> (u16, u16) {
	let border_type = notification.border_type.unwrap_or(BorderType::Plain);

	// Rendering always draws a bordered block.
	let border_v_offset: u16 = 2;
	let border_h_offset: u16 = 2;

	let gutter = gutter_layout(notification.level);
	let effective_padding = padding_with_gutter(notification.padding, gutter);

	let h_padding = effective_padding.left + effective_padding.right;
	let v_padding = effective_padding.top + effective_padding.bottom;

	let min_width = (1 + h_padding + border_h_offset).max(3);
	let min_height = (1 + v_padding + border_v_offset).max(3);

	let max_width_constraint = notification
		.max_width
		.map(|c| match c {
			SizeConstraint::Absolute(w) => w.min(frame_area.width),
			SizeConstraint::Percentage(p) => {
				((frame_area.width as f32 * p.clamp(0.0, 1.0)).ceil() as u16).max(1)
			}
		})
		.unwrap_or(frame_area.width)
		.max(min_width);

	let max_height_constraint = notification
		.max_height
		.map(|c| match c {
			SizeConstraint::Absolute(h) => h.min(frame_area.height),
			SizeConstraint::Percentage(p) => {
				((frame_area.height as f32 * p.clamp(0.0, 1.0)).ceil() as u16).max(1)
			}
		})
		.unwrap_or(frame_area.height)
		.max(min_height);

	let content_max_line_width = notification
		.content
		.lines
		.iter()
		.map(|l: &Line| l.width())
		.max()
		.unwrap_or(0) as u16;

	let title_width = notification.title.as_ref().map_or(0, |t: &Line| t.width()) as u16;
	let title_padding = notification.padding.left + notification.padding.right;

	let width_for_body = (content_max_line_width + border_h_offset + h_padding).max(min_width);
	let width_for_title = (title_width + border_h_offset + title_padding).max(min_width);

	let intrinsic_width = width_for_body.max(width_for_title);
	let final_width = intrinsic_width.min(max_width_constraint);

	// Measure needed text height by actually rendering into a bounded buffer, then
	// scanning the computed text area for the last non-space symbol.
	let mut temp_block = Block::default()
		.borders(Borders::ALL)
		.border_type(border_type)
		.padding(effective_padding);
	if let Some(title) = &notification.title {
		temp_block = temp_block.title(title.clone());
	}

	let buffer_height = max_height_constraint;
	let mut buffer = Buffer::empty(Rect::new(0, 0, final_width, buffer_height));

	let paragraph = Paragraph::new(notification.content.clone())
		.wrap(Wrap { trim: true })
		.block(temp_block.clone());
	paragraph.render(buffer.area, &mut buffer);

	let text_area = temp_block.inner(buffer.area);
	let used_text_height = measure_used_text_height(&buffer, text_area).max(1);

	let needed_height = used_text_height
		.saturating_add(border_v_offset)
		.saturating_add(v_padding);

	let final_height = needed_height.max(min_height).min(max_height_constraint);
	(final_width, final_height)
}

fn measure_used_text_height(buffer: &Buffer, text_area: Rect) -> u16 {
	if text_area.width == 0 || text_area.height == 0 {
		return 0;
	}

	let mut last_used_y: Option<u16> = None;
	for row in 0..text_area.height {
		let y = text_area.y.saturating_add(row);
		let mut row_has_glyph = false;

		for col in 0..text_area.width {
			let x = text_area.x.saturating_add(col);
			let sym = buffer
				.cell((x, y))
				.map(|cell| cell.symbol())
				.unwrap_or("");
			if !sym.is_empty() && sym != " " {
				row_has_glyph = true;
				break;
			}
		}

		if row_has_glyph {
			last_used_y = Some(y);
		}
	}

	match last_used_y {
		Some(y) => y.saturating_sub(text_area.y).saturating_add(1),
		None => 0,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::notifications::types::{Level, SizeConstraint};

	#[test]
	fn calculate_size_includes_gutter_width_when_icon_present() {
		let frame_area = Rect::new(0, 0, 120, 40);

		let mut without_icon = Notification::default();
		without_icon.level = None;
		without_icon.content = Text::raw("1234567890");
		without_icon.max_width = Some(SizeConstraint::Absolute(120));

		let mut with_icon = without_icon.clone();
		with_icon.level = Some(Level::Info);

		let (w0, _) = calculate_size(&without_icon, frame_area);
		let (w1, _) = calculate_size(&with_icon, frame_area);

		assert!(w1 > w0, "expected icon gutter to increase width");
	}

	#[test]
	fn calculate_size_wraps_content_with_gutter() {
		let frame_area = Rect::new(0, 0, 120, 40);

		let mut n = Notification::default();
		n.level = Some(Level::Info);
		// Include whitespace so ratatui wrap can break lines.
		// Use enough words to require >1 wrapped row even with min height.
		n.content = Text::raw("abcde fghij klmno");
		// Force a width where the gutter meaningfully reduces content width.
		n.max_width = Some(SizeConstraint::Absolute(12));
		n.max_height = Some(SizeConstraint::Absolute(40));

		let (_, h) = calculate_size(&n, frame_area);
		assert!(h > 3, "expected wrapping to increase height");
	}

	#[test]
	fn percentage_constraints_round_up() {
		let frame_area = Rect::new(0, 0, 80, 8);

		let mut n = Notification::default();
		n.level = Some(Level::Info);
		n.content = Text::raw("Buffer has unsaved changes (use :write)");

		let (_, h) = calculate_size(&n, frame_area);
		// With ceil rounding and default max_height=0.4, 8*0.4 => 4.
		assert!(h >= 4, "expected room for >1 content line");
	}
}
