mod buffer;
/// Completion popup rendering.
mod completion;
mod document;
/// Status line rendering.
mod status;
/// Line wrapping with sticky punctuation.
pub mod wrap;

pub use buffer::{BufferRenderContext, DiagnosticLineMap, RenderResult, ensure_buffer_cursor_visible};
#[cfg(feature = "lsp")]
pub use buffer::build_diagnostic_line_map;
pub use wrap::{WrapSegment, wrap_line};
