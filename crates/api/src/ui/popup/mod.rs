//! Popup system for cursor-anchored UI elements.
//!
//! This module provides a flexible popup system for displaying context-sensitive
//! UI elements like hover tooltips, completion menus, signature help, and code actions.
//!
//! # Architecture
//!
//! The popup system consists of:
//! - [`Popup`] trait: Interface for popup implementations
//! - [`PopupManager`]: Stack-based manager for showing, dismissing, and rendering popups
//! - [`PopupAnchor`]: Positioning anchor for popups relative to cursor or screen
//! - [`TooltipPopup`]: Simple text popup for hover information

mod anchor;
#[cfg(feature = "lsp")]
mod code_actions;
#[cfg(feature = "lsp")]
mod completion;
#[cfg(feature = "lsp")]
mod hover;
#[cfg(feature = "lsp")]
mod location_picker;
mod manager;
#[cfg(feature = "lsp")]
mod signature;
mod tooltip;

pub use anchor::{PopupAnchor, SizeHints};
#[cfg(feature = "lsp")]
pub use code_actions::{CodeActionResult, CodeActionsPopup};
#[cfg(feature = "lsp")]
pub use completion::{CompletionAcceptResult, CompletionPopup};
#[cfg(feature = "lsp")]
pub use hover::{HoverContent, HoverLine, HoverPopup};
#[cfg(feature = "lsp")]
pub use location_picker::{LocationEntry, LocationPickerPopup};
pub use manager::PopupManager;
#[cfg(feature = "lsp")]
pub use signature::SignaturePopup;
pub use tooltip::TooltipPopup;

use std::any::Any;

use xeno_registry::themes::Theme;
use xeno_tui::layout::Rect;
use xeno_tui::Frame;

/// Events that can be delivered to popups.
#[derive(Debug, Clone)]
pub enum PopupEvent {
    /// Keyboard input event.
    Key(termina::event::KeyEvent),
    /// Mouse input event.
    Mouse(termina::event::MouseEvent),
    /// Cursor moved to a new position.
    CursorMoved,
    /// Popup should be dismissed.
    Dismiss,
}

/// Result returned from popup event handlers.
#[derive(Debug, Default)]
pub struct PopupEventResult {
    /// Whether the event was consumed (stops further propagation).
    pub consumed: bool,
    /// Whether the popup should be dismissed.
    pub dismiss: bool,
}

impl PopupEventResult {
    /// Creates a result indicating the event was consumed.
    pub fn consumed() -> Self {
        Self {
            consumed: true,
            dismiss: false,
        }
    }

    /// Creates a result indicating the event was not consumed.
    pub fn not_consumed() -> Self {
        Self {
            consumed: false,
            dismiss: false,
        }
    }

    /// Creates a result indicating the popup should be dismissed.
    pub fn dismissed() -> Self {
        Self {
            consumed: true,
            dismiss: true,
        }
    }
}

/// Trait for UI popups that can be displayed over the editor content.
///
/// Popups are stacked with later popups rendering on top. Events are routed
/// to the topmost popup first.
pub trait Popup: Send {
    /// Returns the unique identifier for this popup instance.
    fn id(&self) -> &str;

    /// Returns the preferred anchor point for positioning.
    fn anchor(&self) -> PopupAnchor;

    /// Returns the minimum and maximum dimensions.
    fn size_hints(&self) -> SizeHints;

    /// Handles an input event, returning whether it was consumed.
    fn handle_event(&mut self, event: PopupEvent) -> PopupEventResult;

    /// Renders the popup content into the given area.
    fn render(&self, frame: &mut Frame, area: Rect, theme: &Theme);

    /// Whether this popup should capture all input (modal).
    ///
    /// Modal popups prevent events from reaching underlying content until dismissed.
    fn is_modal(&self) -> bool {
        false
    }

    /// Whether this popup should dismiss when the cursor moves.
    ///
    /// Typically true for tooltips, false for menus and pickers.
    fn dismiss_on_cursor_move(&self) -> bool {
        true
    }

    /// Returns a reference to the popup as `Any` for downcasting.
    fn as_any(&self) -> &dyn Any;

    /// Returns a mutable reference to the popup as `Any` for downcasting.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
