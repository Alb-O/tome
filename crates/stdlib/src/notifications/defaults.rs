use std::time::Duration;

use tome_macro::register_notification;
use tome_manifest::SemanticStyle;

use crate::notifications::{AutoDismiss, Level};

register_notification!(
	INFO,
	"info",
	level: Level::Info,
	style: SemanticStyle::Info,
	dismiss: AutoDismiss::After(Duration::from_secs(4))
);

register_notification!(
	WARN,
	"warn",
	level: Level::Warn,
	style: SemanticStyle::Warning,
	dismiss: AutoDismiss::After(Duration::from_secs(6))
);

register_notification!(
	ERROR,
	"error",
	level: Level::Error,
	style: SemanticStyle::Error,
	dismiss: AutoDismiss::Never
);

register_notification!(
	SUCCESS,
	"success",
	level: Level::Info,
	style: SemanticStyle::Success,
	dismiss: AutoDismiss::After(Duration::from_secs(3))
);

register_notification!(
	DEBUG,
	"debug",
	level: Level::Debug,
	style: SemanticStyle::Dim,
	dismiss: AutoDismiss::After(Duration::from_secs(2))
);
