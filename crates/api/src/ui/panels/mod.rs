//! Panel implementations for the editor.
//!
//! Panels are persistent UI elements that can be docked in various positions
//! around the editor (top, bottom, left, right).

#[cfg(feature = "lsp")]
mod diagnostics;
#[cfg(feature = "lsp")]
mod references;

#[cfg(feature = "lsp")]
pub use diagnostics::DiagnosticsPanel;
#[cfg(feature = "lsp")]
pub use references::ReferencesPanel;
