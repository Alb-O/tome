//! Popup manager for stacking and routing events to popups.

use termina::event::{KeyCode, KeyEvent, MouseButton, MouseEventKind};
use xeno_registry::themes::Theme;
use xeno_tui::layout::Rect;
use xeno_tui::Frame;

use super::anchor::calculate_popup_position;
use super::{Popup, PopupEvent};

#[cfg(feature = "lsp")]
use super::CompletionAcceptResult;

/// Manages a stack of popups with event routing and rendering.
///
/// Popups are stacked with later popups rendering on top. Events are routed
/// to the topmost popup first. The manager handles:
///
/// - Showing and dismissing popups by ID
/// - Routing events to the appropriate popup
/// - Rendering popups in stack order
/// - Dismissing on Escape or click outside
#[derive(Default)]
pub struct PopupManager {
    /// Stack of active popups (last = topmost).
    popups: Vec<Box<dyn Popup>>,
    /// Cached areas for click-outside detection.
    rendered_areas: Vec<(String, Rect)>,
}

impl PopupManager {
    /// Creates a new empty popup manager.
    pub fn new() -> Self {
        Self::default()
    }

    /// Shows a popup, adding it to the top of the stack.
    ///
    /// If a popup with the same ID already exists, it is replaced.
    pub fn show(&mut self, popup: Box<dyn Popup>) {
        let id = popup.id().to_string();
        // Remove any existing popup with the same ID
        self.popups.retain(|p| p.id() != id);
        self.popups.push(popup);
    }

    /// Dismisses a popup by ID.
    pub fn dismiss(&mut self, id: &str) {
        self.popups.retain(|p| p.id() != id);
        self.rendered_areas.retain(|(pid, _)| pid != id);
    }

    /// Dismisses all popups.
    pub fn dismiss_all(&mut self) {
        self.popups.clear();
        self.rendered_areas.clear();
    }

    /// Returns whether any popups are currently shown.
    pub fn has_popups(&self) -> bool {
        !self.popups.is_empty()
    }

    /// Returns the number of active popups.
    pub fn popup_count(&self) -> usize {
        self.popups.len()
    }

    /// Returns whether a popup with the given ID is currently shown.
    pub fn has_popup(&self, id: &str) -> bool {
        self.popups.iter().any(|p| p.id() == id)
    }

    /// Gets a reference to a popup by ID, with downcasting.
    pub fn get_popup<T: Popup + 'static>(&self, id: &str) -> Option<&T> {
        self.popups
            .iter()
            .find(|p| p.id() == id)
            .and_then(|p| p.as_any().downcast_ref::<T>())
    }

    /// Gets a mutable reference to a popup by ID, with downcasting.
    pub fn get_popup_mut<T: Popup + 'static>(&mut self, id: &str) -> Option<&mut T> {
        self.popups
            .iter_mut()
            .find(|p| p.id() == id)
            .and_then(|p| p.as_any_mut().downcast_mut::<T>())
    }

    /// Returns whether the topmost popup is modal.
    pub fn is_modal(&self) -> bool {
        self.popups.last().map(|p| p.is_modal()).unwrap_or(false)
    }

    /// Notifies popups that the cursor has moved.
    ///
    /// Popups with `dismiss_on_cursor_move() == true` will be dismissed.
    pub fn notify_cursor_moved(&mut self) {
        let to_dismiss: Vec<String> = self
            .popups
            .iter()
            .filter(|p| p.dismiss_on_cursor_move())
            .map(|p| p.id().to_string())
            .collect();

        for id in to_dismiss {
            self.dismiss(&id);
        }
    }

    /// Checks if a completion popup is active.
    #[cfg(feature = "lsp")]
    pub fn has_completion_popup(&self) -> bool {
        self.popups.iter().any(|p| p.id() == "lsp-completion")
    }

    /// Attempts to accept the currently selected completion item.
    ///
    /// If a completion popup is active and Tab/Enter is pressed, this method:
    /// 1. Gets the acceptance result from the completion popup
    /// 2. Dismisses the completion popup
    /// 3. Returns the acceptance result
    ///
    /// Returns `None` if no completion popup is active, the key is not an accept key,
    /// or no item is selected.
    #[cfg(feature = "lsp")]
    pub fn try_accept_completion(
        &mut self,
        key: &KeyEvent,
    ) -> Option<(CompletionAcceptResult, usize)> {
        // Only handle Tab and Enter
        if !matches!(key.code, KeyCode::Tab | KeyCode::Enter) {
            return None;
        }

        // Find the completion popup index
        let popup_idx = self
            .popups
            .iter()
            .position(|p| p.id() == "lsp-completion")?;

        // Get the acceptance result before removing the popup
        // We need to downcast to CompletionPopup to access accept_selected()
        let result = {
            let popup = &self.popups[popup_idx];
            if let Some(completion_popup) = popup.as_any().downcast_ref::<super::CompletionPopup>() {
                let trigger_column = completion_popup.trigger_column();
                completion_popup.accept_selected().map(|r| (r, trigger_column))
            } else {
                None
            }
        };

        // Remove the popup and clean up
        self.popups.remove(popup_idx);
        self.rendered_areas.retain(|(id, _)| id != "lsp-completion");

        result
    }

    /// Updates the filter text for an active completion popup.
    ///
    /// Returns `true` if a completion popup was found and updated.
    #[cfg(feature = "lsp")]
    pub fn update_completion_filter(&mut self, filter_text: String) -> bool {
        // Find the completion popup
        let popup = self
            .popups
            .iter_mut()
            .find(|p| p.id() == "lsp-completion");

        if let Some(popup) = popup {
            if let Some(completion) = popup.as_any_mut().downcast_mut::<super::CompletionPopup>() {
                completion.set_filter(filter_text);
                return true;
            }
        }
        false
    }

    /// Checks if the completion popup should be dismissed (no matches).
    #[cfg(feature = "lsp")]
    pub fn should_dismiss_completion(&self) -> bool {
        let popup = self
            .popups
            .iter()
            .find(|p| p.id() == "lsp-completion");

        if let Some(popup) = popup {
            if let Some(completion) = popup.as_any().downcast_ref::<super::CompletionPopup>() {
                return !completion.has_items();
            }
        }
        false
    }

    /// Checks if a location picker popup is active.
    #[cfg(feature = "lsp")]
    pub fn has_location_picker(&self) -> bool {
        self.popups.iter().any(|p| p.id() == "lsp-location-picker")
    }

    /// Attempts to accept the currently selected location in the picker.
    ///
    /// If a location picker popup is active and Enter is pressed, this method:
    /// 1. Gets the selected location from the location picker popup
    /// 2. Dismisses the location picker popup
    /// 3. Returns the selected location
    ///
    /// Returns `None` if no location picker popup is active, the key is not Enter,
    /// or no location is selected.
    #[cfg(feature = "lsp")]
    pub fn try_accept_location_picker(
        &mut self,
        key: &KeyEvent,
    ) -> Option<xeno_lsp::lsp_types::Location> {
        // Only handle Enter
        if !matches!(key.code, KeyCode::Enter) {
            return None;
        }

        // Find the location picker popup index
        let popup_idx = self
            .popups
            .iter()
            .position(|p| p.id() == "lsp-location-picker")?;

        // Get the selected location before removing the popup
        let result = {
            let popup = &self.popups[popup_idx];
            if let Some(picker_popup) = popup.as_any().downcast_ref::<super::LocationPickerPopup>() {
                picker_popup.accept_selected()
            } else {
                None
            }
        };

        // Remove the popup and clean up
        self.popups.remove(popup_idx);
        self.rendered_areas.retain(|(id, _)| id != "lsp-location-picker");

        result
    }

    /// Handles a key event, routing to the topmost popup.
    ///
    /// Returns `true` if the event was consumed by a popup.
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        // Escape dismisses the topmost popup
        if key.code == KeyCode::Escape {
            if let Some(popup) = self.popups.last() {
                let id = popup.id().to_string();
                self.dismiss(&id);
                return true;
            }
            return false;
        }

        // Route to topmost popup
        if let Some(popup) = self.popups.last_mut() {
            let result = popup.handle_event(PopupEvent::Key(key));
            if result.dismiss {
                let id = popup.id().to_string();
                self.dismiss(&id);
            }
            return result.consumed;
        }

        false
    }

    /// Handles a mouse event with click-outside detection.
    ///
    /// Returns `true` if the event was consumed by a popup.
    pub fn handle_mouse(&mut self, mouse: termina::event::MouseEvent) -> bool {
        if self.popups.is_empty() {
            return false;
        }

        let click_pos = (mouse.column, mouse.row);

        // Check if click is inside any popup (from topmost to bottommost)
        let mut inside_popup = false;
        let mut target_id: Option<String> = None;

        for (id, area) in self.rendered_areas.iter().rev() {
            if click_pos.0 >= area.x
                && click_pos.0 < area.x + area.width
                && click_pos.1 >= area.y
                && click_pos.1 < area.y + area.height
            {
                inside_popup = true;
                target_id = Some(id.clone());
                break;
            }
        }

        // Click outside dismisses all popups (if it's a press)
        if !inside_popup && matches!(mouse.kind, MouseEventKind::Down(MouseButton::Left)) {
            self.dismiss_all();
            return false; // Let the click through to underlying content
        }

        // Route to the popup that was clicked
        if let Some(id) = target_id
            && let Some(popup) = self.popups.iter_mut().find(|p| p.id() == id)
        {
            let result = popup.handle_event(PopupEvent::Mouse(mouse));
            if result.dismiss {
                self.dismiss(&id);
            }
            return result.consumed;
        }

        // If we have popups and a modal one is on top, consume the event
        self.is_modal()
    }

    /// Renders all popups in stack order (bottom to top).
    ///
    /// # Arguments
    ///
    /// * `frame` - The frame to render into
    /// * `screen` - The available screen area
    /// * `cursor_pos` - The cursor's screen position (x, y), if known
    /// * `theme` - The current theme for styling
    pub fn render(
        &mut self,
        frame: &mut Frame,
        screen: Rect,
        cursor_pos: Option<(u16, u16)>,
        theme: &Theme,
    ) {
        self.rendered_areas.clear();

        for popup in &self.popups {
            let hints = popup.size_hints();
            let anchor = popup.anchor();

            // For now, use preferred size or reasonable defaults
            let content_width = if hints.preferred_width > 0 {
                hints.preferred_width
            } else {
                40 // Default width
            };
            let content_height = if hints.preferred_height > 0 {
                hints.preferred_height
            } else {
                10 // Default height
            };

            let area = calculate_popup_position(
                screen,
                anchor,
                cursor_pos,
                hints,
                content_width,
                content_height,
            );

            popup.render(frame, area, theme);
            self.rendered_areas.push((popup.id().to_string(), area));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::popup::{PopupAnchor, PopupEventResult, SizeHints};

    /// Simple test popup implementation.
    struct TestPopup {
        id: String,
        modal: bool,
        dismiss_on_move: bool,
    }

    impl TestPopup {
        fn new(id: &str) -> Self {
            Self {
                id: id.to_string(),
                modal: false,
                dismiss_on_move: true,
            }
        }

        fn modal(id: &str) -> Self {
            Self {
                id: id.to_string(),
                modal: true,
                dismiss_on_move: false,
            }
        }
    }

    impl Popup for TestPopup {
        fn id(&self) -> &str {
            &self.id
        }

        fn anchor(&self) -> PopupAnchor {
            PopupAnchor::Center
        }

        fn size_hints(&self) -> SizeHints {
            SizeHints::preferred(20, 10)
        }

        fn handle_event(&mut self, _event: PopupEvent) -> PopupEventResult {
            PopupEventResult::not_consumed()
        }

        fn render(&self, _frame: &mut Frame, _area: Rect, _theme: &Theme) {
            // No-op for tests
        }

        fn is_modal(&self) -> bool {
            self.modal
        }

        fn dismiss_on_cursor_move(&self) -> bool {
            self.dismiss_on_move
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
    }

    #[test]
    fn test_show_and_dismiss() {
        let mut manager = PopupManager::new();
        assert!(!manager.has_popups());

        manager.show(Box::new(TestPopup::new("test1")));
        assert!(manager.has_popups());
        assert_eq!(manager.popup_count(), 1);

        manager.dismiss("test1");
        assert!(!manager.has_popups());
    }

    #[test]
    fn test_dismiss_all() {
        let mut manager = PopupManager::new();
        manager.show(Box::new(TestPopup::new("test1")));
        manager.show(Box::new(TestPopup::new("test2")));
        assert_eq!(manager.popup_count(), 2);

        manager.dismiss_all();
        assert!(!manager.has_popups());
    }

    #[test]
    fn test_replace_same_id() {
        let mut manager = PopupManager::new();
        manager.show(Box::new(TestPopup::new("test1")));
        manager.show(Box::new(TestPopup::new("test1")));
        assert_eq!(manager.popup_count(), 1);
    }

    #[test]
    fn test_modal_detection() {
        let mut manager = PopupManager::new();
        assert!(!manager.is_modal());

        manager.show(Box::new(TestPopup::new("test1")));
        assert!(!manager.is_modal());

        manager.show(Box::new(TestPopup::modal("modal1")));
        assert!(manager.is_modal());

        manager.dismiss("modal1");
        assert!(!manager.is_modal());
    }

    #[test]
    fn test_cursor_move_dismissal() {
        let mut manager = PopupManager::new();
        manager.show(Box::new(TestPopup::new("test1")));
        assert!(manager.has_popups());

        manager.notify_cursor_moved();
        assert!(!manager.has_popups());
    }
}
