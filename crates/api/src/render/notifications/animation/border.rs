use tome_manifest::notifications::SlideDirection;

/// Calculates when border effects should be triggered during slide animation.
///
/// Returns a tuple of (trigger_start, trigger_end) representing the progress
/// values at which border modifications should begin and end.
pub fn calculate_triggers(
	slide_direction: SlideDirection,
	actual_start_x: f32,
	actual_start_y: f32,
	actual_end_x: f32,
	actual_end_y: f32,
	frame_x1: f32,
	frame_y1: f32,
	frame_x2: f32,
	frame_y2: f32,
	width: f32,
	height: f32,
) -> (f32, f32) {
	match slide_direction {
		SlideDirection::FromRight => {
			calculate_horizontal_triggers(actual_start_x, actual_end_x, width, frame_x2, true)
		}
		SlideDirection::FromLeft => {
			calculate_horizontal_triggers(actual_start_x, actual_end_x, width, frame_x1, false)
		}
		SlideDirection::FromTop => {
			let crosses = actual_start_y < frame_y1 || actual_end_y < frame_y1;
			if crosses { (0.0, 1.0) } else { (2.0, 0.0) }
		}
		SlideDirection::FromBottom => {
			let crosses = actual_start_y + height > frame_y2 || actual_end_y + height > frame_y2;
			if crosses { (0.0, 1.0) } else { (2.0, 0.0) }
		}
		SlideDirection::FromTopLeft => {
			let cx = actual_start_x < frame_x1 || actual_end_x < frame_x1;
			let cy = actual_start_y < frame_y1 || actual_end_y < frame_y1;
			if cx || cy { (0.0, 1.0) } else { (2.0, 0.0) }
		}
		SlideDirection::FromTopRight => {
			let cx = actual_start_x + width > frame_x2 || actual_end_x + width > frame_x2;
			let cy = actual_start_y < frame_y1 || actual_end_y < frame_y1;
			if cx || cy { (0.0, 1.0) } else { (2.0, 0.0) }
		}
		SlideDirection::FromBottomLeft => {
			let cx = actual_start_x < frame_x1 || actual_end_x < frame_x1;
			let cy = actual_start_y + height > frame_y2 || actual_end_y + height > frame_y2;
			if cx || cy { (0.0, 1.0) } else { (2.0, 0.0) }
		}
		SlideDirection::FromBottomRight => {
			let cx = actual_start_x + width > frame_x2 || actual_end_x + width > frame_x2;
			let cy = actual_start_y + height > frame_y2 || actual_end_y + height > frame_y2;
			if cx || cy { (0.0, 1.0) } else { (2.0, 0.0) }
		}
		SlideDirection::Default | _ => (2.0, 0.0),
	}
}

/// Helper to calculate trigger points for horizontal slides (left/right).
fn calculate_horizontal_triggers(
	start_x: f32,
	end_x: f32,
	width: f32,
	frame_edge: f32,
	is_from_right: bool,
) -> (f32, f32) {
	let crosses = if is_from_right {
		start_x + width > frame_edge || end_x + width > frame_edge
	} else {
		start_x < frame_edge || end_x < frame_edge
	};

	if !crosses {
		return (2.0, 0.0);
	}

	let travel_dist = end_x - start_x;
	let required_pos = if is_from_right {
		frame_edge - width
	} else {
		frame_edge
	};

	let trigger_s = if is_from_right && travel_dist <= 0.0 || !is_from_right && travel_dist >= 0.0 {
		0.0
	} else {
		let dist_to_reach = required_pos - start_x;
		if is_from_right && dist_to_reach <= 0.0 || !is_from_right && dist_to_reach >= 0.0 {
			0.0
		} else {
			(dist_to_reach / travel_dist).clamp(0.0, 1.0)
		}
	};

	let trigger_e = if is_from_right && travel_dist >= 0.0 || !is_from_right && travel_dist <= 0.0 {
		1.0
	} else {
		let required_pos_edge = if is_from_right {
			frame_edge - width - start_x
		} else {
			frame_edge - start_x
		};
		if is_from_right && required_pos_edge >= 0.0 || !is_from_right && required_pos_edge <= 0.0 {
			1.0
		} else {
			(required_pos_edge / travel_dist).clamp(0.0, 1.0)
		}
	};

	(trigger_s, trigger_e)
}
