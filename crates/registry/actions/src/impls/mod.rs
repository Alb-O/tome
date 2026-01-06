/// Diagnostic navigation actions.
pub(crate) mod diagnostic;
/// Text editing actions (delete, change, yank).
pub(crate) mod editing;
/// Search and find actions.
pub(crate) mod find;
/// Insert mode text entry actions.
pub(crate) mod insert;
/// LSP-related actions.
pub(crate) mod lsp;
/// Miscellaneous utility actions.
pub(crate) mod misc;
/// Mode switching actions.
pub(crate) mod modes;
/// Motion-based actions.
pub(crate) mod motions;
/// Command palette actions.
pub mod palette;
/// Key sequence prefix descriptions for which-key HUD.
pub(crate) mod prefixes;
/// Viewport scrolling actions.
pub(crate) mod scroll;
/// Selection manipulation actions.
pub(crate) mod selection_ops;
/// Text object actions.
pub(crate) mod text_objects;
/// Window and split management actions.
pub(crate) mod window;
