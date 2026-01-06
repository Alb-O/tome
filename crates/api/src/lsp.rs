//! LSP integration for the xeno editor.
//!
//! This module bridges the editor's buffer system with LSP functionality,
//! providing document synchronization, diagnostics, and language features.
//!
//! # Feature Flag
//!
//! This module is only available when the `lsp` feature is enabled:
//!
//! ```toml
//! [dependencies]
//! xeno-api = { version = "0.1", features = ["lsp"] }
//! ```
//!
//! # Architecture
//!
//! The LSP integration consists of:
//!
//! - [`LspManager`] - Central coordinator for LSP functionality
//! - Document synchronization via [`xeno_lsp::DocumentSync`]
//! - Server registry via [`xeno_lsp::Registry`]
//!
//! # Usage
//!
//! ```ignore
//! use xeno_api::lsp::LspManager;
//!
//! let lsp = LspManager::new();
//!
//! // Configure language servers
//! lsp.configure_server("rust", LanguageServerConfig {
//!     command: "rust-analyzer".into(),
//!     root_markers: vec!["Cargo.toml".into()],
//!     ..Default::default()
//! });
//!
//! // Open a document
//! lsp.on_buffer_open(&buffer).await?;
//!
//! // Notify of changes
//! lsp.on_buffer_change(&buffer).await?;
//! ```

use std::path::Path;
use std::sync::Arc;

use xeno_lsp::{
	ClientHandle, DocumentStateManager, DocumentSync, LanguageServerConfig, OffsetEncoding,
	Registry, Result,
};

use crate::buffer::Buffer;

/// Central manager for LSP functionality.
///
/// Coordinates language server lifecycle, document synchronization,
/// and provides access to language features.
pub struct LspManager {
	/// Document synchronization coordinator.
	sync: DocumentSync,
}

impl LspManager {
	/// Create a new LSP manager.
	pub fn new() -> Self {
		let registry = Arc::new(Registry::new());
		let documents = Arc::new(DocumentStateManager::new());
		let sync = DocumentSync::new(registry, documents);
		Self { sync }
	}

	/// Create an LSP manager with existing registry and document state.
	pub fn with_state(registry: Arc<Registry>, documents: Arc<DocumentStateManager>) -> Self {
		let sync = DocumentSync::new(registry, documents);
		Self { sync }
	}

	/// Configure a language server.
	pub fn configure_server(&self, language: impl Into<String>, config: LanguageServerConfig) {
		self.sync.registry().register(language, config);
	}

	/// Remove a language server configuration.
	pub fn remove_server(&self, language: &str) {
		self.sync.registry().unregister(language);
	}

	/// Get the document sync coordinator.
	pub fn sync(&self) -> &DocumentSync {
		&self.sync
	}

	/// Get the server registry.
	pub fn registry(&self) -> &Registry {
		self.sync.registry()
	}

	/// Get the document state manager.
	pub fn documents(&self) -> &DocumentStateManager {
		self.sync.documents()
	}

	/// Called when a buffer is opened.
	///
	/// Starts the appropriate language server and opens the document.
	pub async fn on_buffer_open(&self, buffer: &Buffer) -> Result<Option<ClientHandle>> {
		let Some(path) = &buffer.path() else {
			return Ok(None);
		};

		let Some(language) = &buffer.file_type() else {
			return Ok(None);
		};

		// Check if we have a server configured for this language
		if self.sync.registry().get_config(language).is_none() {
			return Ok(None);
		}

		let content = buffer.doc().content.clone();
		let client = self.sync.open_document(path, language, &content).await?;
		Ok(Some(client))
	}

	/// Called when a buffer's content changes.
	///
	/// Sends a full document sync to the language server.
	pub async fn on_buffer_change(&self, buffer: &Buffer) -> Result<()> {
		let Some(path) = &buffer.path() else {
			return Ok(());
		};

		let Some(language) = &buffer.file_type() else {
			return Ok(());
		};

		let content = buffer.doc().content.clone();
		self.sync.notify_change_full(path, language, &content).await
	}

	/// Called when a buffer's content changes with specific range info.
	///
	/// Sends an incremental document sync to the language server.
	pub async fn on_buffer_change_incremental(
		&self,
		buffer: &Buffer,
		start_char: usize,
		end_char: usize,
		new_text: &str,
	) -> Result<()> {
		let Some(path) = &buffer.path() else {
			return Ok(());
		};

		let Some(language) = &buffer.file_type() else {
			return Ok(());
		};

		// Get encoding from the client, default to UTF-16
		let encoding = self.get_encoding_for_path(path, language);

		let content = buffer.doc().content.clone();
		self.sync
			.notify_change_incremental(
				path, language, &content, start_char, end_char, new_text, encoding,
			)
			.await
	}

	/// Called before a buffer is saved.
	pub fn on_buffer_will_save(&self, buffer: &Buffer) -> Result<()> {
		let Some(path) = &buffer.path() else {
			return Ok(());
		};

		let Some(language) = &buffer.file_type() else {
			return Ok(());
		};

		self.sync.notify_will_save(path, language)
	}

	/// Called after a buffer is saved.
	pub fn on_buffer_did_save(&self, buffer: &Buffer, include_text: bool) -> Result<()> {
		let Some(path) = &buffer.path() else {
			return Ok(());
		};

		let Some(language) = &buffer.file_type() else {
			return Ok(());
		};

		let doc = buffer.doc();
		let text = if include_text {
			Some(&doc.content)
		} else {
			None
		};
		self.sync
			.notify_did_save(path, language, include_text, text)
	}

	/// Called when a buffer is closed.
	pub fn on_buffer_close(&self, buffer: &Buffer) -> Result<()> {
		let Some(path) = &buffer.path() else {
			return Ok(());
		};

		let Some(language) = &buffer.file_type() else {
			return Ok(());
		};

		self.sync.close_document(path, language)
	}

	/// Get diagnostics for a buffer.
	pub fn get_diagnostics(&self, buffer: &Buffer) -> Vec<xeno_lsp::lsp_types::Diagnostic> {
		buffer
			.path()
			.as_ref()
			.map(|p| self.sync.get_diagnostics(p))
			.unwrap_or_default()
	}

	/// Get error count for a buffer.
	pub fn error_count(&self, buffer: &Buffer) -> usize {
		buffer
			.path()
			.as_ref()
			.map(|p| self.sync.error_count(p))
			.unwrap_or(0)
	}

	/// Get warning count for a buffer.
	pub fn warning_count(&self, buffer: &Buffer) -> usize {
		buffer
			.path()
			.as_ref()
			.map(|p| self.sync.warning_count(p))
			.unwrap_or(0)
	}

	/// Get total error count across all documents.
	pub fn total_error_count(&self) -> usize {
		self.sync.total_error_count()
	}

	/// Get total warning count across all documents.
	pub fn total_warning_count(&self) -> usize {
		self.sync.total_warning_count()
	}

	/// Get the total diagnostic revision across all active servers.
	///
	/// This counter increases each time any server publishes diagnostics.
	/// Can be used to detect when diagnostics have changed and a redraw is needed.
	pub fn diagnostic_revision(&self) -> u64 {
		self.sync.registry().total_diagnostic_revision()
	}

	/// Get all diagnostics across all documents.
	///
	/// Returns a vector of (URI, diagnostics) pairs for all documents that have diagnostics.
	pub fn all_diagnostics(&self) -> Vec<(xeno_lsp::lsp_types::Url, Vec<xeno_lsp::lsp_types::Diagnostic>)> {
		self.sync.all_diagnostics()
	}

	/// Get a language server client for a buffer.
	pub fn get_client(&self, buffer: &Buffer) -> Option<ClientHandle> {
		let path = buffer.path()?;
		let language = buffer.file_type()?;
		self.sync.registry().get_for_file(&language, &path)
	}

	/// Request hover information at the cursor position.
	pub async fn hover(&self, buffer: &Buffer) -> Result<Option<xeno_lsp::lsp_types::Hover>> {
		let client = match self.get_client(buffer) {
			Some(c) => c,
			None => return Ok(None),
		};

		let path = buffer.path().unwrap();
		let language = buffer.file_type().unwrap();
		let uri = xeno_lsp::lsp_types::Url::from_file_path(&path)
			.map_err(|_| xeno_lsp::Error::Protocol("Invalid path".into()))?;

		let encoding = self.get_encoding_for_path(&path, &language);
		let position =
			xeno_lsp::char_to_lsp_position(&buffer.doc().content, buffer.cursor, encoding)
				.ok_or_else(|| xeno_lsp::Error::Protocol("Invalid position".into()))?;

		client.hover(uri, position).await
	}

	/// Request completions at the cursor position.
	pub async fn completion(
		&self,
		buffer: &Buffer,
	) -> Result<Option<xeno_lsp::lsp_types::CompletionResponse>> {
		let client = match self.get_client(buffer) {
			Some(c) => c,
			None => return Ok(None),
		};

		let path = buffer.path().unwrap();
		let language = buffer.file_type().unwrap();
		let uri = xeno_lsp::lsp_types::Url::from_file_path(&path)
			.map_err(|_| xeno_lsp::Error::Protocol("Invalid path".into()))?;

		let encoding = self.get_encoding_for_path(&path, &language);
		let position =
			xeno_lsp::char_to_lsp_position(&buffer.doc().content, buffer.cursor, encoding)
				.ok_or_else(|| xeno_lsp::Error::Protocol("Invalid position".into()))?;

		client.completion(uri, position, None).await
	}

	/// Request go to definition at the cursor position.
	pub async fn goto_definition(
		&self,
		buffer: &Buffer,
	) -> Result<Option<xeno_lsp::lsp_types::GotoDefinitionResponse>> {
		let client = match self.get_client(buffer) {
			Some(c) => c,
			None => return Ok(None),
		};

		let path = buffer.path().unwrap();
		let language = buffer.file_type().unwrap();
		let uri = xeno_lsp::lsp_types::Url::from_file_path(&path)
			.map_err(|_| xeno_lsp::Error::Protocol("Invalid path".into()))?;

		let encoding = self.get_encoding_for_path(&path, &language);
		let position =
			xeno_lsp::char_to_lsp_position(&buffer.doc().content, buffer.cursor, encoding)
				.ok_or_else(|| xeno_lsp::Error::Protocol("Invalid position".into()))?;

		client.goto_definition(uri, position).await
	}

	/// Request references at the cursor position.
	pub async fn references(
		&self,
		buffer: &Buffer,
		include_declaration: bool,
	) -> Result<Option<Vec<xeno_lsp::lsp_types::Location>>> {
		let client = match self.get_client(buffer) {
			Some(c) => c,
			None => return Ok(None),
		};

		let path = buffer.path().unwrap();
		let language = buffer.file_type().unwrap();
		let uri = xeno_lsp::lsp_types::Url::from_file_path(&path)
			.map_err(|_| xeno_lsp::Error::Protocol("Invalid path".into()))?;

		let encoding = self.get_encoding_for_path(&path, &language);
		let position =
			xeno_lsp::char_to_lsp_position(&buffer.doc().content, buffer.cursor, encoding)
				.ok_or_else(|| xeno_lsp::Error::Protocol("Invalid position".into()))?;

		client.references(uri, position, include_declaration).await
	}

	/// Request formatting for the entire document.
	pub async fn format(
		&self,
		buffer: &Buffer,
	) -> Result<Option<Vec<xeno_lsp::lsp_types::TextEdit>>> {
		let client = match self.get_client(buffer) {
			Some(c) => c,
			None => return Ok(None),
		};

		let path = buffer.path().unwrap();
		let uri = xeno_lsp::lsp_types::Url::from_file_path(&path)
			.map_err(|_| xeno_lsp::Error::Protocol("Invalid path".into()))?;

		// Default formatting options
		let options = xeno_lsp::lsp_types::FormattingOptions {
			tab_size: 4,
			insert_spaces: false,
			..Default::default()
		};

		client.formatting(uri, options).await
	}

	/// Request signature help at the cursor position.
	pub async fn signature_help(
		&self,
		buffer: &Buffer,
		context: Option<xeno_lsp::lsp_types::SignatureHelpContext>,
	) -> Result<Option<xeno_lsp::lsp_types::SignatureHelp>> {
		let client = match self.get_client(buffer) {
			Some(c) => c,
			None => return Ok(None),
		};

		let path = buffer.path().unwrap();
		let language = buffer.file_type().unwrap();
		let uri = xeno_lsp::lsp_types::Url::from_file_path(&path)
			.map_err(|_| xeno_lsp::Error::Protocol("Invalid path".into()))?;

		let encoding = self.get_encoding_for_path(&path, &language);
		let position =
			xeno_lsp::char_to_lsp_position(&buffer.doc().content, buffer.cursor, encoding)
				.ok_or_else(|| xeno_lsp::Error::Protocol("Invalid position".into()))?;

		client.signature_help(uri, position, context).await
	}

	/// Request code actions for the current line or selection.
	///
	/// If a range is provided, code actions are requested for that range.
	/// Otherwise, code actions are requested for the current line.
	pub async fn code_actions(
		&self,
		buffer: &Buffer,
		range: Option<xeno_base::range::Range>,
	) -> Result<Option<xeno_lsp::lsp_types::CodeActionResponse>> {
		let client = match self.get_client(buffer) {
			Some(c) => c,
			None => return Ok(None),
		};

		let path = buffer.path().unwrap();
		let language = buffer.file_type().unwrap();
		let uri = xeno_lsp::lsp_types::Url::from_file_path(&path)
			.map_err(|_| xeno_lsp::Error::Protocol("Invalid path".into()))?;

		let encoding = self.get_encoding_for_path(&path, &language);

		// Convert range to LSP range
		let lsp_range = if let Some(r) = range {
			let start =
				xeno_lsp::char_to_lsp_position(&buffer.doc().content, r.from(), encoding)
					.ok_or_else(|| xeno_lsp::Error::Protocol("Invalid start position".into()))?;
			let end = xeno_lsp::char_to_lsp_position(&buffer.doc().content, r.to(), encoding)
				.ok_or_else(|| xeno_lsp::Error::Protocol("Invalid end position".into()))?;
			xeno_lsp::lsp_types::Range { start, end }
		} else {
			// Use current line range
			let line = buffer.cursor_line();
			let line_start = buffer.doc().content.line_to_char(line);
			let line_end = if line + 1 < buffer.doc().content.len_lines() {
				buffer.doc().content.line_to_char(line + 1)
			} else {
				buffer.doc().content.len_chars()
			};

			let start =
				xeno_lsp::char_to_lsp_position(&buffer.doc().content, line_start, encoding)
					.ok_or_else(|| xeno_lsp::Error::Protocol("Invalid start position".into()))?;
			let end = xeno_lsp::char_to_lsp_position(&buffer.doc().content, line_end, encoding)
				.ok_or_else(|| xeno_lsp::Error::Protocol("Invalid end position".into()))?;
			xeno_lsp::lsp_types::Range { start, end }
		};

		// Build the code action context with diagnostics for this range
		let diagnostics: Vec<xeno_lsp::lsp_types::Diagnostic> = self
			.get_diagnostics(buffer)
			.into_iter()
			.filter(|d| ranges_overlap(&d.range, &lsp_range))
			.collect();

		let context = xeno_lsp::lsp_types::CodeActionContext {
			diagnostics,
			only: None,
			trigger_kind: Some(xeno_lsp::lsp_types::CodeActionTriggerKind::INVOKED),
		};

		client.code_action(uri, lsp_range, context).await
	}

	/// Request inlay hints for a range in the buffer.
	///
	/// If no range is provided, requests hints for the entire visible viewport.
	/// Inlay hints show type annotations, parameter names, and other inferred
	/// information as virtual text.
	pub async fn inlay_hints(
		&self,
		buffer: &Buffer,
		start_line: usize,
		end_line: usize,
	) -> Result<Option<Vec<xeno_lsp::lsp_types::InlayHint>>> {
		let client = match self.get_client(buffer) {
			Some(c) => c,
			None => return Ok(None),
		};

		let path = buffer.path().unwrap();
		let language = buffer.file_type().unwrap();
		let uri = xeno_lsp::lsp_types::Url::from_file_path(&path)
			.map_err(|_| xeno_lsp::Error::Protocol("Invalid path".into()))?;

		let encoding = self.get_encoding_for_path(&path, &language);

		// Convert line range to LSP range
		let content = &buffer.doc().content;
		let start_char = content.line_to_char(start_line);
		let end_char = if end_line < content.len_lines() {
			content.line_to_char(end_line)
		} else {
			content.len_chars()
		};

		let start_pos = xeno_lsp::char_to_lsp_position(content, start_char, encoding)
			.ok_or_else(|| xeno_lsp::Error::Protocol("Invalid start position".into()))?;
		let end_pos = xeno_lsp::char_to_lsp_position(content, end_char, encoding)
			.ok_or_else(|| xeno_lsp::Error::Protocol("Invalid end position".into()))?;

		let range = xeno_lsp::lsp_types::Range {
			start: start_pos,
			end: end_pos,
		};

		client.inlay_hints(uri, range).await
	}

	/// Shutdown all language servers.
	pub async fn shutdown_all(&self) {
		self.sync.registry().shutdown_all().await;
	}

	/// Get the offset encoding for a language server.
	fn get_encoding_for_path(&self, path: &Path, language: &str) -> OffsetEncoding {
		self.sync
			.registry()
			.get_for_file(language, path)
			.map(|c| c.offset_encoding())
			.unwrap_or_default()
	}
}

impl Default for LspManager {
	fn default() -> Self {
		Self::new()
	}
}

/// Checks if two LSP ranges overlap.
fn ranges_overlap(a: &xeno_lsp::lsp_types::Range, b: &xeno_lsp::lsp_types::Range) -> bool {
	// Two ranges overlap if neither ends before the other starts
	!(a.end.line < b.start.line
		|| (a.end.line == b.start.line && a.end.character < b.start.character)
		|| b.end.line < a.start.line
		|| (b.end.line == a.start.line && b.end.character < a.start.character))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_lsp_manager_creation() {
		let manager = LspManager::new();
		assert_eq!(manager.total_error_count(), 0);
		assert_eq!(manager.total_warning_count(), 0);
	}

	#[test]
	fn test_configure_server() {
		let manager = LspManager::new();
		manager.configure_server(
			"rust",
			LanguageServerConfig {
				command: "rust-analyzer".into(),
				root_markers: vec!["Cargo.toml".into()],
				..Default::default()
			},
		);

		assert!(manager.registry().get_config("rust").is_some());
		assert!(manager.registry().get_config("python").is_none());
	}
}
