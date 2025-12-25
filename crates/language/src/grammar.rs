//! Grammar loading and search path configuration.
//!
//! Grammars are compiled tree-sitter parsers loaded from shared libraries.
//! This module handles locating and loading grammar files.
//!
//! # Runtime Path Discovery
//!
//! The search paths follow a priority order (highest to lowest):
//!
//! 1. `TOME_RUNTIME` environment variable (development)
//! 2. User config directory (`~/.config/tome/`)
//! 3. User data directory (`~/.local/share/tome/`)
//! 4. Helix config directory (`~/.config/helix/runtime/`) - compatibility fallback
//! 5. Next to executable (`./grammars/`, `../share/tome/grammars/`)
//!
//! This allows Tome to reuse Helix's pre-built grammars and queries if available,
//! providing a "just works" experience for users who already have Helix installed.

use std::path::{Path, PathBuf};

use thiserror::Error;
use tree_house::tree_sitter::Grammar;

/// Errors that can occur when loading a grammar.
#[derive(Error, Debug)]
pub enum GrammarError {
	#[error("grammar not found: {0}")]
	NotFound(String),

	#[error("failed to load grammar library: {0}")]
	LoadError(String),

	#[error("grammar library missing language function: {0}")]
	MissingSymbol(String),

	#[error("IO error: {0}")]
	Io(#[from] std::io::Error),
}

/// Loads a grammar by name from the search paths.
///
/// Searches all configured grammar directories for a matching shared library.
/// If the grammar is not found, returns `GrammarError::NotFound`.
///
/// For automatic fetching/building of missing grammars, use [`load_grammar_or_build`].
pub fn load_grammar(name: &str) -> Result<Grammar, GrammarError> {
	let lib_name = grammar_library_name(name);

	for path in grammar_search_paths() {
		let lib_path = path.join(&lib_name);

		if lib_path.exists() {
			return load_grammar_from_path(&lib_path, name);
		}
	}

	Err(GrammarError::NotFound(name.to_string()))
}

/// Loads a grammar by name, automatically fetching and building if necessary.
///
/// This function first tries to load the grammar from the search paths.
/// If not found and `languages.toml` contains a configuration for this grammar,
/// it will attempt to fetch the source and compile it.
///
/// This provides a "just works" experience where grammars are built on demand.
pub fn load_grammar_or_build(name: &str) -> Result<Grammar, GrammarError> {
	// First, try to load from existing paths
	match load_grammar(name) {
		Ok(grammar) => return Ok(grammar),
		Err(GrammarError::NotFound(_)) => {
			// Grammar not found, try to build it
			log::info!(
				"Grammar '{}' not found, attempting to fetch and build...",
				name
			);
		}
		Err(e) => return Err(e),
	}

	// Try to auto-build the grammar
	if let Err(e) = auto_build_grammar(name) {
		log::warn!("Failed to auto-build grammar '{}': {}", name, e);
		return Err(GrammarError::NotFound(name.to_string()));
	}

	// Try loading again after building
	load_grammar(name)
}

/// Fetches grammar source from git and compiles it to a shared library.
///
/// Looks up the grammar configuration in `languages.toml`, clones/fetches
/// the source repository, and invokes the C compiler to build the `.so` file.
fn auto_build_grammar(name: &str) -> Result<(), GrammarError> {
	use crate::build::{build_grammar, fetch_grammar, load_grammar_configs};

	let configs = load_grammar_configs().map_err(|e| {
		GrammarError::Io(std::io::Error::new(
			std::io::ErrorKind::Other,
			e.to_string(),
		))
	})?;

	let config = configs
		.into_iter()
		.find(|c| c.grammar_id == name)
		.ok_or_else(|| GrammarError::NotFound(format!("{} (no config in languages.toml)", name)))?;

	log::info!("Fetching grammar source for '{}'...", name);
	fetch_grammar(&config).map_err(|e| {
		GrammarError::Io(std::io::Error::new(
			std::io::ErrorKind::Other,
			format!("fetch failed: {}", e),
		))
	})?;

	log::info!("Building grammar '{}'...", name);
	build_grammar(&config).map_err(|e| {
		GrammarError::Io(std::io::Error::new(
			std::io::ErrorKind::Other,
			format!("build failed: {}", e),
		))
	})?;

	log::info!("Successfully built grammar '{}'", name);
	Ok(())
}

/// Loads a grammar from a specific library path.
fn load_grammar_from_path(path: &Path, name: &str) -> Result<Grammar, GrammarError> {
	// SAFETY: Loading a tree-sitter grammar from a dynamic library.
	// The library must contain a valid tree-sitter language function.
	unsafe {
		Grammar::new(name, path)
			.map_err(|e| GrammarError::LoadError(format!("{}: {}", path.display(), e)))
	}
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

/// Source for loading a grammar.
#[derive(Debug, Clone)]
pub enum GrammarSource {
	/// Load from a shared library at the given path.
	Library(PathBuf),
	/// Use a pre-compiled grammar (future: for bundled grammars).
	Builtin(&'static str),
}

/// Returns runtime directories where grammars are searched.
///
/// Priority order:
/// 1. `TOME_RUNTIME` env var (development)
/// 2. User config directory (`~/.config/tome/grammars/`)
/// 3. User data directory (`~/.local/share/tome/grammars/`)
/// 4. Helix runtime directories (compatibility fallback)
/// 5. Bundled grammars relative to executable
pub fn grammar_search_paths() -> Vec<PathBuf> {
	let mut dirs = Vec::new();

	// Development: check TOME_RUNTIME env var first
	if let Ok(runtime) = std::env::var("TOME_RUNTIME") {
		dirs.push(PathBuf::from(runtime).join("grammars"));
	}

	// User config directory: ~/.config/tome/grammars/
	if let Some(config_dir) = config_dir() {
		dirs.push(config_dir.join("tome").join("grammars"));
	}

	// User data directory: ~/.local/share/tome/grammars/
	if let Some(data_dir) = data_local_dir() {
		dirs.push(data_dir.join("tome").join("grammars"));
	}

	// Helix compatibility: check Helix runtime directories
	for helix_dir in helix_runtime_dirs() {
		dirs.push(helix_dir.join("grammars"));
	}

	// Bundled grammars relative to executable
	if let Ok(exe_path) = std::env::current_exe()
		&& let Some(exe_dir) = exe_path.parent()
	{
		dirs.push(exe_dir.join("grammars"));
		// Also check ../share/tome/grammars for installed packages
		dirs.push(
			exe_dir
				.join("..")
				.join("share")
				.join("tome")
				.join("grammars"),
		);
	}

	dirs
}

/// Returns the primary runtime directory for Tome.
///
/// Used for storing grammars, queries, and other runtime data.
/// Checks `TOME_RUNTIME` env var first, then falls back to XDG data/config dirs.
pub fn runtime_dir() -> PathBuf {
	if let Ok(runtime) = std::env::var("TOME_RUNTIME") {
		return PathBuf::from(runtime);
	}

	if let Some(data_dir) = data_local_dir() {
		return data_dir.join("tome");
	}

	if let Some(config_dir) = config_dir() {
		return config_dir.join("tome");
	}

	PathBuf::from(".")
}

/// Returns directories to search for query files.
///
/// Priority order:
/// 1. `TOME_RUNTIME` env var (development)
/// 2. User config directory (`~/.config/tome/queries/`)
/// 3. User data directory (`~/.local/share/tome/queries/`)
/// 4. Helix runtime directories (compatibility fallback)
/// 5. Bundled queries relative to executable
pub fn query_search_paths() -> Vec<PathBuf> {
	let mut dirs = Vec::new();

	if let Ok(runtime) = std::env::var("TOME_RUNTIME") {
		dirs.push(PathBuf::from(runtime).join("queries"));
	}

	if let Some(config) = config_dir() {
		dirs.push(config.join("tome").join("queries"));
	}

	if let Some(data) = data_local_dir() {
		dirs.push(data.join("tome").join("queries"));
	}

	for helix_dir in helix_runtime_dirs() {
		dirs.push(helix_dir.join("queries"));
	}

	if let Ok(exe) = std::env::current_exe()
		&& let Some(dir) = exe.parent()
	{
		dirs.push(dir.join("queries"));
		dirs.push(dir.join("..").join("share").join("tome").join("queries"));
	}

	dirs
}

// Minimal platform-specific directory helpers
fn config_dir() -> Option<PathBuf> {
	#[cfg(unix)]
	{
		std::env::var_os("XDG_CONFIG_HOME")
			.map(PathBuf::from)
			.or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
	}
	#[cfg(windows)]
	{
		std::env::var_os("APPDATA").map(PathBuf::from)
	}
	#[cfg(not(any(unix, windows)))]
	{
		None
	}
}

fn data_local_dir() -> Option<PathBuf> {
	#[cfg(unix)]
	{
		std::env::var_os("XDG_DATA_HOME")
			.map(PathBuf::from)
			.or_else(|| {
				std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local").join("share"))
			})
	}
	#[cfg(windows)]
	{
		std::env::var_os("LOCALAPPDATA").map(PathBuf::from)
	}
	#[cfg(not(any(unix, windows)))]
	{
		None
	}
}

/// Returns the cache directory for Tome.
/// Used for storing fetched grammar sources before compilation.
pub fn cache_dir() -> Option<PathBuf> {
	#[cfg(unix)]
	{
		std::env::var_os("XDG_CACHE_HOME")
			.map(PathBuf::from)
			.or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cache")))
			.map(|p| p.join("tome"))
	}
	#[cfg(windows)]
	{
		// Windows uses LOCALAPPDATA for cache-like data
		std::env::var_os("LOCALAPPDATA").map(|p| PathBuf::from(p).join("tome").join("cache"))
	}
	#[cfg(not(any(unix, windows)))]
	{
		None
	}
}

/// Returns directories where Helix stores its runtime files.
/// Used as a fallback to reuse Helix's pre-built grammars and queries.
fn helix_runtime_dirs() -> Vec<PathBuf> {
	let mut dirs = Vec::new();

	// Check HELIX_RUNTIME env var first
	if let Ok(runtime) = std::env::var("HELIX_RUNTIME") {
		dirs.push(PathBuf::from(runtime));
	}

	// Helix config directory: ~/.config/helix/runtime/
	if let Some(config) = config_dir() {
		let helix_config_runtime = config.join("helix").join("runtime");
		if helix_config_runtime.exists() {
			dirs.push(helix_config_runtime);
		}
	}

	// Helix data directory (where grammars are actually built)
	if let Some(data) = data_local_dir() {
		let helix_data_runtime = data.join("helix").join("runtime");
		if helix_data_runtime.exists() {
			dirs.push(helix_data_runtime);
		}
	}

	dirs
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_search_paths_not_empty() {
		// Should have at least the exe-relative path
		let dirs = grammar_search_paths();
		assert!(!dirs.is_empty());
	}

	#[test]
	fn test_query_search_paths_not_empty() {
		let dirs = query_search_paths();
		assert!(!dirs.is_empty());
	}

	#[test]
	fn test_grammar_library_name() {
		let name = grammar_library_name("rust");
		#[cfg(target_os = "linux")]
		assert_eq!(name, "librust.so");
		#[cfg(target_os = "macos")]
		assert_eq!(name, "librust.dylib");
		#[cfg(target_os = "windows")]
		assert_eq!(name, "rust.dll");
	}

	#[test]
	fn test_grammar_library_name_with_dash() {
		// Dashes should be converted to underscores
		let name = grammar_library_name("tree-sitter-rust");
		#[cfg(target_os = "linux")]
		assert_eq!(name, "libtree_sitter_rust.so");
	}

	#[test]
	fn test_cache_dir_is_some() {
		// Should return a valid cache directory on most systems
		let cache = cache_dir();
		// On most Unix systems this should work
		#[cfg(unix)]
		assert!(cache.is_some());
	}

	#[test]
	fn test_helix_runtime_dirs_returns_vec() {
		// Should return a Vec (possibly empty if Helix is not installed)
		let dirs = helix_runtime_dirs();
		// Just verify it doesn't panic
		let _ = dirs.len();
	}
}
