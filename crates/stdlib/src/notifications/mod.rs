//! Notification data model and type registration.
//!
//! This module provides the abstract `Notification` data model and
//! convenience traits for creating notifications. The actual rendering
//! is handled by `tome_api`.
//!
//! # Architecture
//!
//! - **Data model** (`notification/`): Abstract `Notification` struct using
//!   types from `tome_base` and `tome_manifest`. No UI dependencies.
//!
//! - **Type registration** (`defaults.rs`): Registers built-in notification
//!   types (info, warn, error, etc.) and creates convenience extension traits.
//!
//! - **Type re-exports** (`types.rs`): Re-exports types from `tome_manifest`
//!   for convenience.
//!
//! # Usage
//!
//! ```ignore
//! use tome_stdlib::notifications::{Notification, NotificationBuilder};
//!
//! let notif = NotificationBuilder::from_registry("info", "Hello!")
//!     .title("Greeting")
//!     .build()?;
//! ```

mod defaults;
mod notification;
mod types;

// Re-export notification data model
pub use notification::{MAX_CONTENT_CHARS, Notification, NotificationBuilder};

// Re-export convenience traits from defaults
pub use defaults::{
	NotifyDEBUGExt, NotifyERRORExt, NotifyINFOExt, NotifySUCCESSExt, NotifyWARNExt,
};

// Re-export types from manifest
pub use types::*;

// Re-export registry lookup
pub use tome_manifest::notifications::{NOTIFICATION_TYPES, NotificationTypeDef, find_notification_type};
