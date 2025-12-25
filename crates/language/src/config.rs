//! Language configuration for syntax parsing.
//!
//! This module defines the configuration structures that connect file types
//! to their tree-sitter grammars and query files, implementing the
//! `tree_house::LanguageLoader` trait.

use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;

use once_cell::sync::OnceCell;
use tree_house::tree_sitter::Grammar;
use tree_house::{InjectionLanguageMarker, Language, LanguageConfig as TreeHouseConfig};

use crate::grammar::GrammarError;

// Re-export tree_house::Language for convenience.
pub type LanguageId = Language;

/// Language data with lazily-loaded syntax configuration.
///
/// Each registered language has its grammar and queries loaded on first use.
#[derive(Debug)]
pub struct LanguageData {
	/// Language name (e.g., "rust", "python").
	pub name: String,
	/// Grammar name (may differ from language name).
	pub grammar_name: String,
	/// File extensions (without dot).
	pub extensions: Vec<String>,
	/// Exact filenames to match.
	pub filenames: Vec<String>,
	/// Shebang interpreters.
	pub shebangs: Vec<String>,
	/// Comment token(s) for the language.
	pub comment_tokens: Vec<String>,
	/// Block comment tokens (start, end).
	pub block_comment: Option<(String, String)>,
	/// Injection regex for matching in code blocks.
	pub injection_regex: Option<regex::Regex>,
	/// Lazily-loaded syntax configuration.
	config: OnceCell<Option<TreeHouseConfig>>,
}

impl LanguageData {
	/// Creates new language data.
	pub fn new(
		name: String,
		grammar_name: Option<String>,
		extensions: Vec<String>,
		filenames: Vec<String>,
		shebangs: Vec<String>,
		comment_tokens: Vec<String>,
		block_comment: Option<(String, String)>,
		injection_regex: Option<&str>,
	) -> Self {
		Self {
			grammar_name: grammar_name.unwrap_or_else(|| name.clone()),
			name,
			extensions,
			filenames,
			shebangs,
			comment_tokens,
			block_comment,
			injection_regex: injection_regex.and_then(|r| regex::Regex::new(r).ok()),
			config: OnceCell::new(),
		}
	}

	/// Returns the syntax configuration, loading it if necessary.
	///
	/// This loads the grammar and compiles the queries on first access.
	pub fn syntax_config(&self) -> Option<&TreeHouseConfig> {
		self.config
			.get_or_init(|| load_language_config(&self.grammar_name))
			.as_ref()
	}
}

/// Loads the complete language configuration (grammar + queries).
fn load_language_config(grammar_name: &str) -> Option<TreeHouseConfig> {
	let grammar = load_grammar(grammar_name)?;

	let highlights = read_query(grammar_name, "highlights.scm");
	let injections = read_query(grammar_name, "injections.scm");
	let locals = read_query(grammar_name, "locals.scm");

	match TreeHouseConfig::new(grammar, &highlights, &injections, &locals) {
		Ok(config) => Some(config),
		Err(e) => {
			log::warn!(
				"Failed to create language config for {}: {}",
				grammar_name,
				e
			);
			None
		}
	}
}

/// Loads a grammar from the search paths.
fn load_grammar(name: &str) -> Option<Grammar> {
	for path in crate::grammar::grammar_search_paths() {
		let lib_name = grammar_library_name(name);
		let lib_path = path.join(&lib_name);

		if lib_path.exists() {
			match unsafe { load_grammar_from_library(&lib_path, name) } {
				Ok(grammar) => return Some(grammar),
				Err(e) => {
					log::warn!("Failed to load grammar {} from {:?}: {}", name, lib_path, e);
				}
			}
		}
	}

	log::debug!("Grammar not found: {}", name);
	None
}

/// Returns the platform-specific library name for a grammar.
fn grammar_library_name(name: &str) -> String {
	let safe_name = name.replace('-', "_");
	#[cfg(target_os = "macos")]
	{
		format!("lib{safe_name}.dylib")
	}
	#[cfg(target_os = "windows")]
	{
		format!("{safe_name}.dll")
	}
	#[cfg(not(any(target_os = "macos", target_os = "windows")))]
	{
		format!("lib{safe_name}.so")
	}
}

/// Loads a grammar from a shared library.
///
/// # Safety
/// This loads and executes code from a dynamic library.
unsafe fn load_grammar_from_library(path: &Path, name: &str) -> Result<Grammar, GrammarError> {
	// SAFETY: The caller ensures the library path is valid and contains a tree-sitter grammar
	unsafe {
		Grammar::new(name, path).map_err(|e| GrammarError::LoadError(format!("{}: {}", path.display(), e)))
	}
}

/// Reads a query file for a language.
///
/// This searches the query paths and handles the `; inherits` directive
/// using tree-house's `read_query` helper.
pub fn read_query(lang: &str, filename: &str) -> String {
	tree_house::read_query(lang, |query_lang| {
		for path in crate::grammar::query_search_paths() {
			let query_path = path.join(query_lang).join(filename);
			if let Ok(content) = std::fs::read_to_string(&query_path) {
				return content;
			}
		}
		String::new()
	})
}

/// The main language loader that implements tree-house's LanguageLoader trait.
///
/// This is the central registry for all language configurations. It handles:
/// - Language registration with file type associations
/// - Lazy loading of grammars and queries
/// - Lookup by filename, extension, shebang, or name
#[derive(Debug, Default)]
pub struct LanguageLoader {
	/// All registered languages.
	languages: Vec<LanguageData>,
	/// Lookup by extension.
	by_extension: HashMap<String, usize>,
	/// Lookup by filename.
	by_filename: HashMap<String, usize>,
	/// Lookup by shebang.
	by_shebang: HashMap<String, usize>,
	/// Lookup by name.
	by_name: HashMap<String, usize>,
}

impl LanguageLoader {
	/// Creates a new empty loader.
	pub fn new() -> Self {
		Self::default()
	}

	/// Registers a language.
	pub fn register(&mut self, data: LanguageData) -> Language {
		let idx = self.languages.len();

		for ext in &data.extensions {
			self.by_extension.insert(ext.clone(), idx);
		}

		for fname in &data.filenames {
			self.by_filename.insert(fname.clone(), idx);
		}

		for shebang in &data.shebangs {
			self.by_shebang.insert(shebang.clone(), idx);
		}

		self.by_name.insert(data.name.clone(), idx);

		self.languages.push(data);
		Language::new(idx as u32)
	}

	/// Gets language data by index.
	pub fn get(&self, lang: Language) -> Option<&LanguageData> {
		self.languages.get(lang.idx())
	}

	/// Finds a language by name.
	pub fn language_for_name(&self, name: &str) -> Option<Language> {
		self.by_name.get(name).map(|&idx| Language::new(idx as u32))
	}

	/// Finds a language by file path.
	pub fn language_for_path(&self, path: &Path) -> Option<Language> {
		// Check exact filename first
		if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
			if let Some(&idx) = self.by_filename.get(name) {
				return Some(Language::new(idx as u32));
			}
		}

		// Check extension
		path.extension()
			.and_then(|ext| ext.to_str())
			.and_then(|ext| self.by_extension.get(ext))
			.map(|&idx| Language::new(idx as u32))
	}

	/// Finds a language by shebang line.
	pub fn language_for_shebang(&self, first_line: &str) -> Option<Language> {
		if !first_line.starts_with("#!") {
			return None;
		}

		let line = first_line.trim_start_matches("#!");
		let parts: Vec<&str> = line.split_whitespace().collect();

		// Handle /usr/bin/env python style
		let interpreter = if parts.first() == Some(&"/usr/bin/env") || parts.first() == Some(&"env")
		{
			parts.get(1).copied()
		} else {
			parts.first().and_then(|p| p.rsplit('/').next())
		};

		interpreter.and_then(|interp| {
			// Strip version numbers (python3 -> python)
			let base = interp.trim_end_matches(|c: char| c.is_ascii_digit());
			self.by_shebang
				.get(base)
				.map(|&idx| Language::new(idx as u32))
		})
	}

	/// Finds a language by injection regex match.
	fn language_for_injection_match(&self, text: &str) -> Option<Language> {
		for (idx, lang) in self.languages.iter().enumerate() {
			if let Some(ref regex) = lang.injection_regex {
				if regex.is_match(text) {
					return Some(Language::new(idx as u32));
				}
			}
		}
		None
	}

	/// Finds a language by injection regex match from a RopeSlice.
	fn language_for_injection_rope(&self, text: ropey::RopeSlice<'_>) -> Option<Language> {
		// Convert to Cow<str> for regex matching
		let cow: std::borrow::Cow<str> = text.into();
		self.language_for_injection_match(&cow)
	}

	/// Returns all registered languages.
	pub fn languages(&self) -> impl Iterator<Item = (Language, &LanguageData)> {
		self.languages
			.iter()
			.enumerate()
			.map(|(idx, data)| (Language::new(idx as u32), data))
	}

	/// Returns the number of registered languages.
	pub fn len(&self) -> usize {
		self.languages.len()
	}

	/// Returns true if no languages are registered.
	pub fn is_empty(&self) -> bool {
		self.languages.is_empty()
	}
}

impl tree_house::LanguageLoader for LanguageLoader {
	fn language_for_marker(&self, marker: InjectionLanguageMarker) -> Option<Language> {
		match marker {
			InjectionLanguageMarker::Name(name) => self.language_for_name(name),
			InjectionLanguageMarker::Match(text) => self.language_for_injection_rope(text),
			InjectionLanguageMarker::Filename(text) => {
				let path: Cow<str> = text.into();
				self.language_for_path(Path::new(path.as_ref()))
			}
			InjectionLanguageMarker::Shebang(text) => {
				let line: Cow<str> = text.into();
				self.language_for_shebang(&line)
			}
		}
	}

	fn get_config(&self, lang: Language) -> Option<&TreeHouseConfig> {
		self.languages.get(lang.idx())?.syntax_config()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_loader_registration() {
		let mut loader = LanguageLoader::new();

		let data = LanguageData::new(
			"rust".to_string(),
			None,
			vec!["rs".to_string()],
			vec![],
			vec![],
			vec!["//".to_string()],
			Some(("/*".to_string(), "*/".to_string())),
			None,
		);

		let lang = loader.register(data);
		assert_eq!(lang.idx(), 0);

		let found = loader.language_for_path(Path::new("test.rs"));
		assert_eq!(found, Some(lang));

		let found = loader.language_for_name("rust");
		assert_eq!(found, Some(lang));
	}

	#[test]
	fn test_shebang_detection() {
		let mut loader = LanguageLoader::new();

		let data = LanguageData::new(
			"python".to_string(),
			None,
			vec!["py".to_string()],
			vec![],
			vec!["python".to_string()],
			vec!["#".to_string()],
			None,
			None,
		);

		let lang = loader.register(data);

		assert_eq!(loader.language_for_shebang("#!/usr/bin/python"), Some(lang));
		assert_eq!(
			loader.language_for_shebang("#!/usr/bin/env python"),
			Some(lang)
		);
		assert_eq!(
			loader.language_for_shebang("#!/usr/bin/python3"),
			Some(lang)
		);
		assert_eq!(loader.language_for_shebang("not a shebang"), None);
	}

	#[test]
	fn test_read_query_not_found() {
		// Should return empty string for non-existent query
		let query = read_query("nonexistent_language_xyz", "highlights.scm");
		assert!(query.is_empty());
	}
}
