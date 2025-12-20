use std::collections::HashMap;
use std::time::Instant;

use ratatui::prelude::*;

use crate::notifications::functions::fnc_calculate_anchor_position::calculate_anchor_position;
use crate::notifications::functions::fnc_calculate_rect::calculate_rect;
use crate::notifications::types::{Anchor, AnimationPhase};

/// Vertical spacing between stacked notifications
const STACKING_VERTICAL_SPACING: u16 = 1;

/// Represents a notification with its calculated stacked position
#[derive(Debug, Clone)]
pub struct StackedNotification {
	pub id: u64,
	pub rect: Rect,
}

/// Trait for notification state that can be stacked.
///
/// This trait allows the stacking orchestrator to work with any notification state
/// implementation that provides the necessary information.
pub trait StackableNotification {
	fn id(&self) -> u64;
	fn current_phase(&self) -> AnimationPhase;
	fn created_at(&self) -> Instant;
	fn full_rect(&self) -> Rect;
	fn exterior_padding(&self) -> u16;
	/// Calculate the notification's content size based on frame area.
	/// Returns (width, height) tuple.
	fn calculate_content_size(&self, frame_area: Rect) -> (u16, u16);
}

/// Calculate stacking positions for notifications at a given anchor.
///
/// This function implements the core stacking algorithm:
/// 1. Filters to visible notifications (excludes Pending and Finished)
/// 2. Sorts by creation time (oldest first)
/// 3. Applies max_concurrent limit (keeps newest N)
/// 4. Determines stacking direction based on anchor
/// 5. Calculates accumulated heights and positions
/// 6. Returns list of (id, final_stacked_rect) pairs
///
/// # Arguments
///
/// * `notifications` - HashMap of all notification states
/// * `anchor` - The anchor position for this group
/// * `ids_at_anchor` - List of notification IDs at this anchor
/// * `frame_area` - The available frame area
/// * `max_concurrent` - Optional limit on concurrent visible notifications
///
/// # Returns
///
/// Vec of StackedNotification with calculated positions
///
/// # Type Parameters
///
/// * `T` - Any type implementing StackableNotification trait
pub fn calculate_stacking_positions<T: StackableNotification>(
	notifications: &HashMap<u64, T>,
	anchor: Anchor,
	ids_at_anchor: &[u64],
	frame_area: Rect,
	max_concurrent: Option<usize>,
) -> Vec<StackedNotification> {
	// 1. Filter to visible states and collect data (ID, Creation Time, Calculated Height, Width)
	let mut visible_states_data: Vec<(u64, Instant, u16, u16)> = ids_at_anchor
		.iter()
		.filter_map(|id| {
			notifications.get(id).and_then(|state| {
				let phase = state.current_phase();
				if phase != AnimationPhase::Finished && phase != AnimationPhase::Pending {
					let rect = state.full_rect();
					let (width, height) = if rect.height > 0 && rect.width > 0 {
						(rect.width, rect.height)
					} else {
						// Calculate size from content if not yet set
						state.calculate_content_size(frame_area)
					};
					if height > 0 {
						Some((*id, state.created_at(), height, width))
					} else {
						None
					}
				} else {
					None
				}
			})
		})
		.collect();

	// 2. Sort by creation time (oldest first)
	visible_states_data.sort_unstable_by_key(|&(_, created_at, _, _)| created_at);

	// 3. Apply max_concurrent limit (take the newest N items)
	let max_concurrent = max_concurrent.unwrap_or(usize::MAX);
	let num_to_render = visible_states_data.len().min(max_concurrent);
	let candidate_data = &visible_states_data[visible_states_data.len() - num_to_render..];

	// 4. Determine stacking direction & available height
	let is_stacking_up = matches!(
		anchor,
		Anchor::BottomLeft | Anchor::BottomCenter | Anchor::BottomRight
	);
	let anchor_pos = calculate_anchor_position(anchor, frame_area);
	let available_height = if is_stacking_up {
		anchor_pos.y.saturating_sub(frame_area.y)
	} else {
		frame_area.bottom().saturating_sub(anchor_pos.y)
	};

	// 5. Calculate stack positions and filter by fit
	//
	// Strategy: Iterate in visual order based on anchor position.
	// For bottom anchors (stacking up): iterate newest-to-oldest so newest appears at anchor
	// For top anchors (stacking down): iterate oldest-to-newest so oldest appears at anchor
	let mut accumulated_height: u16 = 0;
	let mut result_list: Vec<StackedNotification> = Vec::with_capacity(num_to_render);

	// Create iterator in correct order for visual stacking
	let iter_order: Box<dyn Iterator<Item = &(u64, Instant, u16, u16)>> = if is_stacking_up {
		Box::new(candidate_data.iter().rev()) // Newest first visually appears at bottom
	} else {
		Box::new(candidate_data.iter()) // Oldest first visually appears at top
	};

	for &(id, _, height, width) in iter_order {
		let spacing = if accumulated_height > 0 {
			STACKING_VERTICAL_SPACING
		} else {
			0
		};
		let offset = accumulated_height.saturating_add(spacing);
		let needed_height = height.saturating_add(spacing);

		if accumulated_height.saturating_add(needed_height) <= available_height {
			// Get the notification state to calculate base rect
			if let Some(state) = notifications.get(&id) {
				// Calculate base rect (X position and unstacked Y)
				let base_full_rect = calculate_rect(
					anchor,
					anchor_pos,
					width,
					height,
					frame_area,
					state.exterior_padding(),
				);

				// Calculate stacked Y based on accumulated height of items already placed
				// For bottom anchors: newer (later) items stack upward (subtract from base Y)
				// For top anchors: newer (later) items stack downward (add to base Y)
				let stacked_y = if is_stacking_up {
					base_full_rect.y.saturating_sub(offset)
				} else {
					base_full_rect.y.saturating_add(offset)
				};

				// Create the final Rect for this notification
				let final_stacked_rect = Rect {
					x: base_full_rect.x,
					y: stacked_y
						.max(frame_area.y)
						.min(frame_area.bottom().saturating_sub(height)),
					width: base_full_rect.width,
					height,
				}
				.intersection(frame_area);

				if final_stacked_rect.width > 0 && final_stacked_rect.height > 0 {
					result_list.push(StackedNotification {
						id,
						rect: final_stacked_rect,
					});
					accumulated_height = accumulated_height.saturating_add(needed_height);
				} else {
					break; // Break if clamping resulted in zero size
				}
			}
		} else {
			// Doesn't fit, stop adding notifications for this anchor
			break;
		}
	}

	result_list
}

#[cfg(test)]
mod tests {
	use super::*;

	struct TestNotification {
		id: u64,
		height: u16,
		created_at: Instant,
	}

	impl StackableNotification for TestNotification {
		fn id(&self) -> u64 {
			self.id
		}
		fn current_phase(&self) -> AnimationPhase {
			AnimationPhase::Dwelling
		}
		fn created_at(&self) -> Instant {
			self.created_at
		}
		fn full_rect(&self) -> Rect {
			Rect::default()
		}
		fn exterior_padding(&self) -> u16 {
			0
		}
		fn calculate_content_size(&self, _frame_area: Rect) -> (u16, u16) {
			(10, self.height)
		}
	}

	#[test]
	fn test_stacking_gap_between_first_two() {
		let frame_area = Rect::new(0, 0, 100, 50);
		let mut notifications = HashMap::new();
		let now = Instant::now();

		// Notif 0 (oldest)
		notifications.insert(
			0,
			TestNotification {
				id: 0,
				height: 3,
				created_at: now,
			},
		);
		// Notif 1 (newer)
		notifications.insert(
			1,
			TestNotification {
				id: 1,
				height: 3,
				created_at: now + std::time::Duration::from_millis(10),
			},
		);

		let ids = vec![0, 1];
		let stacked = calculate_stacking_positions(
			&notifications,
			Anchor::BottomRight,
			&ids,
			frame_area,
			None,
		);

		assert_eq!(stacked.len(), 2);

		// In BottomRight, newest (id: 1) is at the bottom.
		// anchor_pos.y is 49.
		// Notif 1 (height 3): base_y = 49 - (3-1) = 47.
		assert_eq!(stacked[0].id, 1);
		assert_eq!(stacked[0].rect.y, 47);

		// Notif 0 (second newest): base_y = 47.
		// With fix, it should be at 47 - (3 + 1) = 43.
		assert_eq!(stacked[1].id, 0);
		assert_eq!(stacked[1].rect.y, 43);

		// Verify gap: Notif 0 ends at 43+3=46. Notif 1 starts at 47. Gap at line 46.
		assert_eq!(stacked[1].rect.bottom(), 46);
		assert_eq!(stacked[0].rect.y, 47);
	}

	#[test]
	fn test_stacking_gap_consistent_for_three() {
		let frame_area = Rect::new(0, 0, 100, 50);
		let mut notifications = HashMap::new();
		let now = Instant::now();

		for i in 0..3 {
			notifications.insert(
				i,
				TestNotification {
					id: i,
					height: 2,
					created_at: now + std::time::Duration::from_millis(i * 10),
				},
			);
		}

		let ids = vec![0, 1, 2];
		let stacked = calculate_stacking_positions(
			&notifications,
			Anchor::BottomRight,
			&ids,
			frame_area,
			None,
		);

		assert_eq!(stacked.len(), 3);

		// Newest (id: 2) at bottom
		assert_eq!(stacked[0].id, 2);
		assert_eq!(stacked[0].rect.y, 48); // 49 - (2-1) = 48. Rect: 48, 49.

		// Next (id: 1) above it
		assert_eq!(stacked[1].id, 1);
		assert_eq!(stacked[1].rect.y, 48 - (2 + 1)); // 45. Rect: 45, 46. (Gap at 47)
		assert_eq!(stacked[1].rect.y, 45);

		// Oldest (id: 0) above it
		assert_eq!(stacked[2].id, 0);
		assert_eq!(stacked[2].rect.y, 45 - (2 + 1)); // 42. Rect: 42, 43. (Gap at 44)
		assert_eq!(stacked[2].rect.y, 42);
	}
}
