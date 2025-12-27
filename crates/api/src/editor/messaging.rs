use crate::editor::Editor;

impl Editor {
	pub fn notify(&mut self, type_name: &str, text: impl Into<String>) {
		use tome_stdlib::notifications::{NotificationBuilder, find_notification_type};
		let text = text.into();

		let type_def = find_notification_type(type_name);
		let builder = NotificationBuilder::from_registry(type_name, text);

		// Resolve semantic style from theme (with inheritance)
		let semantic = type_def
			.map(|t| t.semantic)
			.unwrap_or(tome_manifest::SEMANTIC_INFO);
		let style = self.theme.colors.notification_style(semantic);

		if let Ok(notif) = builder.style(style).build() {
			let _ = self.notifications.add(notif);
		}
	}
}
