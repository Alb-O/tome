mod buffer;
/// Completion popup rendering.
mod completion;
mod document;
/// Status line rendering.
mod status;
/// Rendering types: wrap segments, render results.
pub mod types;

pub use buffer::{BufferRenderContext, ensure_buffer_cursor_visible};
#[cfg(feature = "lsp")]
pub use buffer::{PreparedDiagnostics, prepare_diagnostics};
#[cfg(feature = "lsp")]
pub use buffer::{PreparedInlayHints, prepare_inlay_hints};
pub use types::{RenderResult, WrapSegment, wrap_line};
