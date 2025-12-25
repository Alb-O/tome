use ratatui::layout::Rect;
pub use tome_manifest::notifications::*;

/// Animation phase tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationPhase {
	#[default]
	Pending,
	SlidingIn,
	Expanding,
	FadingIn,
	Dwelling,
	SlidingOut,
	Collapsing,
	FadingOut,
	Finished,
}

/// Behavior when notification limit is reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Overflow {
	#[default]
	DiscardOldest,
	DiscardNewest,
}

/// Constraint on notification dimensions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SizeConstraint {
	Absolute(u16),
	Percentage(f32),
}

/// Direction from which a notification slides in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[non_exhaustive]
pub enum SlideDirection {
	#[default]
	Default,
	FromTop,
	FromBottom,
	FromLeft,
	FromRight,
	FromTopLeft,
	FromTopRight,
	FromBottomLeft,
	FromBottomRight,
}

/// Parameters for sliding animations.
#[derive(Debug, Clone, Copy)]
pub struct SlideParams {
	pub full_rect: Rect,
	pub frame_area: Rect,
	pub progress: f32,
	pub phase: AnimationPhase,
	pub anchor: Anchor,
	pub slide_direction: SlideDirection,
	pub custom_slide_in_start_pos: Option<(f32, f32)>,
	pub custom_slide_out_end_pos: Option<(f32, f32)>,
}
