//! Tracing layer that writes log events to the debug panel's ring buffer.

use tracing::field::{Field, Visit};
use tracing::{Event, Level, Subscriber};

use super::ring_buffer::{LOG_BUFFER, LogEntry, LogLevel};

/// A tracing layer that writes events to the debug panel's log buffer.
pub struct DebugPanelLayer;

impl DebugPanelLayer {
	pub fn new() -> Self {
		Self
	}
}

impl Default for DebugPanelLayer {
	fn default() -> Self {
		Self::new()
	}
}

struct MessageVisitor(String);

impl Visit for MessageVisitor {
	fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
		if field.name() == "message" {
			self.0 = format!("{:?}", value);
		} else if self.0.is_empty() {
			self.0 = format!("{}={:?}", field.name(), value);
		}
	}

	fn record_str(&mut self, field: &Field, value: &str) {
		if field.name() == "message" {
			self.0 = value.to_string();
		} else if self.0.is_empty() {
			self.0 = format!("{}={}", field.name(), value);
		}
	}
}

impl<S: Subscriber> tracing_subscriber::Layer<S> for DebugPanelLayer {
	fn on_event(&self, event: &Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
		let level = match *event.metadata().level() {
			Level::ERROR => LogLevel::Error,
			Level::WARN => LogLevel::Warn,
			Level::INFO => LogLevel::Info,
			Level::DEBUG => LogLevel::Debug,
			Level::TRACE => LogLevel::Trace,
		};

		let mut visitor = MessageVisitor(String::new());
		event.record(&mut visitor);

		let message = if visitor.0.is_empty() {
			event.metadata().name().to_string()
		} else {
			visitor.0
		};

		LOG_BUFFER.push(LogEntry {
			level,
			target: event.metadata().target().to_string(),
			message,
		});
	}
}
