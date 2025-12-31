//! Drop-down menu bar widget.
//!
//! A horizontal menu bar with keyboard-navigable dropdown menus, similar to
//! traditional desktop application menus.
//!
//! # Example
//!
//! ```ignore
//! use evildoer_tui::widgets::menu::{Menu, MenuState, MenuItem};
//!
//! let mut state = MenuState::new(vec![
//!     MenuItem::group("File", vec![
//!         MenuItem::item("New", "file:new"),
//!         MenuItem::item("Open", "file:open"),
//!         MenuItem::item("Save", "file:save"),
//!     ]),
//!     MenuItem::group("Edit", vec![
//!         MenuItem::item("Undo", "edit:undo"),
//!         MenuItem::item("Redo", "edit:redo"),
//!     ]),
//! ]);
//!
//! // Activate menu (select first item)
//! state.activate();
//!
//! // Navigate with arrow keys
//! state.right(); // Move to next top-level item
//! state.down();  // Open dropdown / move down in dropdown
//! state.select(); // Select current item
//!
//! // Drain events after each frame
//! for event in state.drain_events() {
//!     match event {
//!         MenuEvent::Selected(action) => handle_action(action),
//!     }
//! }
//! ```

mod item;
mod state;
mod widget;

pub use item::MenuItem;
pub use state::{MenuEvent, MenuState};
pub use widget::Menu;
