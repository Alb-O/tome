//! Buffer rendering for split views.
//!
//! This module provides buffer-agnostic rendering that can render any buffer
//! given a `BufferRenderContext`. This enables proper split view rendering
//! where multiple buffers are rendered simultaneously.

mod context;
#[cfg(feature = "lsp")]
mod diagnostics;
mod gutter;
#[cfg(feature = "lsp")]
mod inlay_hints;
mod viewport;

pub use context::BufferRenderContext;
#[cfg(feature = "lsp")]
pub use diagnostics::{DiagnosticDisplay, LineDiagnostics, PreparedDiagnostics, prepare_diagnostics};
#[cfg(feature = "lsp")]
pub use inlay_hints::{InlayHintDisplay, InlayHintDisplayKind, LineInlayHints, PreparedInlayHints, prepare_inlay_hints};
pub use viewport::ensure_buffer_cursor_visible;
