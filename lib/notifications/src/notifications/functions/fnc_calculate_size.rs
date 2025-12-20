use ratatui::buffer::{Buffer, Cell};
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
pub fn calculate_size(notification: &Notification, frame_area: Rect) -> (u16, u16) {
	let border_v_offset = match notification.border_type {
		Some(BorderType::Double) => 2,
		Some(_) => 2,
		None => 0,
	};
	let border_h_offset = match notification.border_type {
		Some(BorderType::Double) => 2,
		Some(_) => 2,
		None => 0,
	};

	let gutter = gutter_layout(notification.level);
	let effective_padding = padding_with_gutter(notification.padding, gutter);

	let body_h_padding = effective_padding.left + effective_padding.right;
	let body_v_padding = effective_padding.top + effective_padding.bottom;

	let min_width = (1 + body_h_padding + border_h_offset).max(3);
	let min_height = (1 + body_v_padding + border_v_offset).max(3);

	let max_width_constraint = notification
		.max_width
		.map(|c| match c {
			SizeConstraint::Absolute(w) => w.min(frame_area.width),
			SizeConstraint::Percentage(p) => {
				((frame_area.width as f32 * p.clamp(0.0, 1.0)) as u16).max(1)
			}
		})
		.unwrap_or(frame_area.width)
		.max(min_width);

	let content_max_line_width = notification
		.content
		.lines
		.iter()
		.map(|l: &Line| l.width())
		.max()
		.unwrap_or(0) as u16;

	let title_width = notification.title.as_ref().map_or(0, |t: &Line| t.width()) as u16;
	let title_padding = notification.padding.left + notification.padding.right;

	let width_for_body = (content_max_line_width + border_h_offset + body_h_padding).max(min_width);
	let width_for_title = (title_width + border_h_offset + title_padding).max(min_width);

	let intrinsic_width = width_for_body.max(width_for_title);
	let final_width = intrinsic_width.min(max_width_constraint);

	let max_height_constraint = notification
		.max_height
		.map(|c| match c {
			SizeConstraint::Absolute(h) => h.min(frame_area.height),
			SizeConstraint::Percentage(p) => {
				((frame_area.height as f32 * p.clamp(0.0, 1.0)) as u16).max(1)
			}
		})
		.unwrap_or(frame_area.height)
		.max(min_height);

	let mut temp_block = Block::default();
	if let Some(border_type) = notification.border_type {
		temp_block = temp_block.borders(Borders::ALL).border_type(border_type);
	}
	if let Some(title) = &notification.title {
		temp_block = temp_block.title(title.clone());
	}
	// Account for the gutter/icon column by treating it as extra left padding.
	temp_block = temp_block.padding(effective_padding);

	let temp_paragraph = Paragraph::new(notification.content.clone())
		.wrap(Wrap { trim: true })
		.block(temp_block);

	let buffer_height = max_height_constraint;
	let mut buffer = Buffer::empty(Rect::new(0, 0, final_width, buffer_height));
	temp_paragraph.render(buffer.area, &mut buffer);

	let default_cell = Cell::default();
	let measured_height = buffer
		.content
		.iter()
		.enumerate()
		.filter(|(_, cell)| *cell != &default_cell)
		.map(|(idx, _)| buffer.pos_of(idx).1)
		.max()
		.map_or(0, |row_index| row_index + 1);

	let final_height = measured_height.max(min_height).min(max_height_constraint);
	(final_width, final_height)
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
}
