pub mod dock;
mod focus;
pub mod keymap;
mod manager;
/// Panel traits and request types.
pub mod panel;
/// Panel implementations (diagnostics, references, etc.).
pub mod panels;
/// Popup system for cursor-anchored UI elements.
pub mod popup;

pub use focus::FocusTarget;
pub use keymap::UiKeyChord;
pub use manager::UiManager;
pub use panel::UiRequest;
#[cfg(feature = "lsp")]
pub use panels::DiagnosticsPanel;
#[cfg(feature = "lsp")]
pub use panels::ReferencesPanel;
#[cfg(feature = "lsp")]
pub use popup::{CompletionAcceptResult, CompletionPopup, HoverContent, HoverLine, HoverPopup};
pub use popup::{Popup, PopupAnchor, PopupEvent, PopupEventResult, PopupManager, SizeHints};
