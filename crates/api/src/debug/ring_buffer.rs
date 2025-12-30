//! Thread-safe ring buffer for log entries.

use std::collections::VecDeque;
use std::sync::{LazyLock, RwLock};

/// Maximum number of log entries to retain.
pub const MAX_LOG_ENTRIES: usize = 1000;

/// Global log buffer instance.
pub static LOG_BUFFER: LazyLock<LogRingBuffer> = LazyLock::new(LogRingBuffer::new);

/// Log severity levels, ordered from least to most severe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
	Trace,
	Debug,
	Info,
	Warn,
	Error,
}

impl From<tracing::Level> for LogLevel {
	fn from(level: tracing::Level) -> Self {
		match level {
			tracing::Level::ERROR => LogLevel::Error,
			tracing::Level::WARN => LogLevel::Warn,
			tracing::Level::INFO => LogLevel::Info,
			tracing::Level::DEBUG => LogLevel::Debug,
			tracing::Level::TRACE => LogLevel::Trace,
		}
	}
}

/// A single log entry.
#[derive(Debug, Clone)]
pub struct LogEntry {
	pub level: LogLevel,
	pub target: String,
	pub message: String,
}

/// Thread-safe ring buffer for log entries.
pub struct LogRingBuffer {
	entries: RwLock<VecDeque<LogEntry>>,
}

impl LogRingBuffer {
	pub fn new() -> Self {
		Self {
			entries: RwLock::new(VecDeque::with_capacity(MAX_LOG_ENTRIES)),
		}
	}

	/// Pushes a new log entry, evicting the oldest if at capacity.
	pub fn push(&self, entry: LogEntry) {
		let mut entries = self.entries.write().unwrap();
		if entries.len() >= MAX_LOG_ENTRIES {
			entries.pop_front();
		}
		entries.push_back(entry);
	}

	pub fn entries(&self) -> Vec<LogEntry> {
		self.entries.read().unwrap().iter().cloned().collect()
	}

	pub fn len(&self) -> usize {
		self.entries.read().unwrap().len()
	}

	pub fn is_empty(&self) -> bool {
		self.entries.read().unwrap().is_empty()
	}

	pub fn clear(&self) {
		self.entries.write().unwrap().clear();
	}
}

impl Default for LogRingBuffer {
	fn default() -> Self {
		Self::new()
	}
}
