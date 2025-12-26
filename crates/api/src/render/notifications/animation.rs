mod border;
mod expand;
mod fade;
mod slide;

pub use expand::calculate_rect as expand_calculate_rect;
pub use fade::{FadeHandler, calculate_rect as fade_calculate_rect, interpolate_color};
pub use slide::{
	apply_border_effect as slide_apply_border_effect, calculate_rect as slide_calculate_rect,
	offscreen_position as slide_offscreen_position, resolve_direction as resolve_slide_direction,
};
