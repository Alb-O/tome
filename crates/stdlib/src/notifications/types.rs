//! Notification types re-exported from tome-manifest.
//!
//! This module provides convenient access to notification-related types.
//! All types are defined in `tome_manifest::notifications` to keep them
//! UI-agnostic.

pub use tome_manifest::notifications::{
	Anchor, Animation, AnimationPhase, AutoDismiss, Level, NotificationError, Overflow,
	SizeConstraint, SlideDirection, Timing,
};
