use tome_manifest::completion::{CommandSource, CompletionContext, CompletionSource};
use tome_theme::ThemeSource;

use crate::editor::Editor;
use crate::editor::types::{Message, MessageKind};

impl Editor {
	#[allow(
		dead_code,
		reason = "Method currently unused but intended for future extension use cases"
	)]
	pub fn request_redraw(&mut self) {
		self.needs_redraw = true;
	}

	pub fn notify(&mut self, type_name: &str, text: impl Into<String>) {
		use tome_stdlib::notifications::{
			Level as NotifLevel, NotificationBuilder, find_notification_type,
		};
		let text = text.into();

		// Update legacy message field for CLI and status line
		let type_def = find_notification_type(type_name);
		let kind = match type_def.map(|t| t.level).unwrap_or(NotifLevel::Info) {
			NotifLevel::Error => MessageKind::Error,
			NotifLevel::Warn => MessageKind::Warning,
			_ => MessageKind::Info,
		};
		self.message = Some(Message {
			text: text.clone(),
			kind,
		});

		let builder = NotificationBuilder::from_registry(type_name, text);

		// Resolve semantic style from theme (with inheritance)
		let semantic = type_def
			.map(|t| t.semantic)
			.unwrap_or(tome_manifest::SEMANTIC_INFO);
		let style = self.theme.colors.notification_style(semantic);

		if let Ok(notif) = builder.style(style.into()).build() {
			let _ = self.notifications.add(notif);
		}
	}

	pub fn update_completions(&mut self) {
		if let Some((prompt, input)) = self.input.command_line() {
			let ctx = CompletionContext {
				input: input.to_string(),
				cursor: input.len(),
				prompt,
			};

			// Query completion sources in priority order
			// First source with results wins (determines replace_start)
			let sources: &[&dyn CompletionSource] = &[&ThemeSource, &CommandSource];

			let mut result = None;
			for source in sources {
				let r = source.complete(&ctx);
				if !r.is_empty() {
					result = Some(r);
					break;
				}
			}

			if let Some(r) = result {
				self.completions.replace_start = r.start;
				self.completions.items = r.items;
				self.completions.items.sort_by(|a, b| a.label.cmp(&b.label));
				self.completions.items.dedup_by(|a, b| a.label == b.label);
				self.completions.active = true;
			} else {
				self.completions.active = false;
				self.completions.items.clear();
				self.completions.replace_start = 0;
			}

			// Keep selection if still valid, otherwise reset
			if let Some(idx) = self.completions.selected_idx
				&& idx >= self.completions.items.len()
			{
				self.completions.selected_idx = None;
				self.completions.scroll_offset = 0;
			}
		} else {
			self.completions.active = false;
			self.completions.items.clear();
			self.completions.selected_idx = None;
			self.completions.replace_start = 0;
			self.completions.scroll_offset = 0;
		}
	}
}
