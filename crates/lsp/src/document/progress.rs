//! Progress tracking for LSP operations.
//!
//! Tracks work-done progress notifications from language servers.

use std::time::Instant;

use lsp_types::NumberOrString;

use crate::client::LanguageServerId;

/// An active progress operation from a language server.
#[derive(Debug, Clone)]
pub struct ProgressItem {
	/// Server that reported this progress.
	pub server_id: LanguageServerId,
	/// Progress token for tracking.
	pub token: NumberOrString,
	/// Title of the operation (e.g., "Indexing").
	pub title: String,
	/// Optional message with more details.
	pub message: Option<String>,
	/// Optional percentage (0-100).
	pub percentage: Option<u32>,
	/// When this progress started.
	pub started_at: Instant,
}
