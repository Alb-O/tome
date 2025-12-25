pub mod animation;
pub mod defaults;
pub mod layout;
pub mod manager;
pub mod notification;
pub mod render;
pub mod stacking;
pub mod state;
pub mod types;
pub mod ui;
pub mod utils;

pub use defaults::{
	NotifyDEBUGExt, NotifyERRORExt, NotifyINFOExt, NotifySUCCESSExt, NotifyWARNExt,
};
pub use manager::Notifications;
pub use notification::{Notification, NotificationBuilder, calculate_size, generate_code};
pub use tome_manifest::notifications::{
	Animation, AutoDismiss, Level, NOTIFICATION_TYPES, NotificationTypeDef, Timing,
	find_notification_type,
};
pub use types::*;
