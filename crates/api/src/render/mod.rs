mod buffer;
/// Completion popup rendering.
mod completion;
mod document;
/// Status line rendering.
mod status;
/// Line wrapping with sticky punctuation.
pub mod wrap;

pub use buffer::{BufferRenderContext, RenderResult, ensure_buffer_cursor_visible};
pub use wrap::{WrapSegment, wrap_line};
