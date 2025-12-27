use tome_manifest::notifications as manifest;
use tome_stdlib::notifications::find_notification_type;
use tome_tui::widgets::notifications::{self as notif, Toast};

use crate::editor::Editor;

impl Editor {
	pub fn notify(&mut self, type_name: &str, text: impl Into<String>) {
		let text = text.into();
		let type_def = find_notification_type(type_name);

		let semantic = type_def
			.map(|t| t.semantic)
			.unwrap_or(tome_manifest::SEMANTIC_INFO);
		let style: tome_tui::style::Style = self.theme.colors.notification_style(semantic).into();

		let mut toast = Toast::new(text).style(style).border_style(style);

		if let Some(def) = type_def {
			toast = toast
				.animation(match def.animation {
					manifest::Animation::Slide => notif::Animation::Slide,
					manifest::Animation::ExpandCollapse => notif::Animation::ExpandCollapse,
					manifest::Animation::Fade => notif::Animation::Fade,
				})
				.auto_dismiss(match def.auto_dismiss {
					manifest::AutoDismiss::Never => notif::AutoDismiss::Never,
					manifest::AutoDismiss::After(d) => notif::AutoDismiss::After(d),
				});
		}

		self.notifications.push(toast);
	}
}
