//! Integration tests for tome-language syntax highlighting.
//!
//! These tests verify the complete pipeline from language registration
//! through syntax parsing to highlight span generation.
//!
//! NOTE: Full syntax highlighting requires compiled tree-sitter grammars.
//! Without grammars, tests verify the API works but can't produce highlights.
//! To get grammars, run: `TOME_RUNTIME=runtime tome grammar fetch && tome grammar build`

use ropey::Rope;
use tome_language::grammar::{grammar_search_paths, load_grammar};
use tome_language::highlight::{Highlight, HighlightStyles};
use tome_language::syntax::Syntax;
use tome_language::{LanguageData, LanguageLoader};

/// Creates a simple test language loader with Rust registered.
fn create_test_loader() -> LanguageLoader {
	let mut loader = LanguageLoader::new();

	let rust = LanguageData::new(
		"rust".to_string(),
		None, // grammar_name defaults to language name
		vec!["rs".to_string()],
		vec![],
		vec![],
		vec!["//".to_string()],
		Some(("/*".to_string(), "*/".to_string())),
		Some("rust"),
	);

	loader.register(rust);
	loader
}

#[test]
fn test_language_registration() {
	let loader = create_test_loader();

	assert!(
		loader.language_for_name("rust").is_some(),
		"Should find rust language by name"
	);

	assert!(
		loader
			.language_for_path(std::path::Path::new("test.rs"))
			.is_some(),
		"Should find rust language by .rs extension"
	);

	assert!(
		loader.language_for_name("unknown").is_none(),
		"Should not find unknown language"
	);
}

#[test]
fn test_language_data_fields() {
	let loader = create_test_loader();

	let lang = loader.language_for_name("rust").unwrap();
	let data = loader.get(lang).unwrap();

	assert_eq!(data.name, "rust");
	assert_eq!(data.grammar_name, "rust");
	assert_eq!(data.extensions, vec!["rs"]);
	assert_eq!(data.comment_tokens, vec!["//"]);
	assert_eq!(
		data.block_comment,
		Some(("/*".to_string(), "*/".to_string()))
	);
}

#[test]
fn test_syntax_config_loading() {
	let loader = create_test_loader();

	let lang = loader.language_for_name("rust").unwrap();
	let data = loader.get(lang).unwrap();

	// Try to load syntax config - this will fail if grammar isn't installed
	// but we can at least verify the method exists and doesn't panic
	let config = data.syntax_config();

	// Log whether we have a grammar available
	if config.is_some() {
		println!("Rust grammar loaded successfully!");
	} else {
		println!("Rust grammar not available (expected in CI without grammars)");
	}
}

#[test]
fn test_highlight_styles_creation() {
	let scopes = ["keyword", "function", "string", "comment"];

	let styles = HighlightStyles::new(&scopes, |scope| {
		use ratatui::style::{Color, Style};
		match scope {
			"keyword" => Style::default().fg(Color::Red),
			"function" => Style::default().fg(Color::Blue),
			"string" => Style::default().fg(Color::Green),
			"comment" => Style::default().fg(Color::Gray),
			_ => Style::default(),
		}
	});

	assert_eq!(styles.len(), 4);
	assert!(!styles.is_empty());
}

#[test]
fn test_highlight_styles_resolution() {
	use ratatui::style::{Color, Style};

	let scopes = ["keyword", "function"];

	let styles = HighlightStyles::new(&scopes, |scope| match scope {
		"keyword" => Style::default().fg(Color::Red),
		"function" => Style::default().fg(Color::Blue),
		_ => Style::default(),
	});

	let keyword_style = styles.style_for_highlight(Highlight::new(0));
	let function_style = styles.style_for_highlight(Highlight::new(1));
	let unknown_style = styles.style_for_highlight(Highlight::new(99));

	assert_eq!(keyword_style.fg, Some(Color::Red));
	assert_eq!(function_style.fg, Some(Color::Blue));
	assert_eq!(unknown_style.fg, None); // Out of bounds returns default
}

#[test]
fn test_syntax_creation_without_grammar() {
	let loader = create_test_loader();
	let source = Rope::from_str("fn main() { println!(\"Hello\"); }");

	let lang = loader.language_for_name("rust").unwrap();

	// Try to create syntax - may fail without grammar
	let syntax = Syntax::new(source.slice(..), lang, &loader);

	if let Ok(syntax) = syntax {
		println!("Syntax created successfully!");

		// Verify we can access the tree
		let tree = syntax.tree();
		println!("Parse tree root: {:?}", tree.root_node().kind());
	} else {
		println!(
			"Syntax creation failed (expected without grammar): {:?}",
			syntax.err()
		);
	}
}

#[test]
fn test_grammar_loading_debug() {
	// Debug test to understand grammar loading
	println!("Grammar search paths:");
	for path in grammar_search_paths() {
		println!("  {:?} (exists: {})", path, path.exists());
		let grammar_path = path.join("rust.so");
		println!(
			"    rust.so: {:?} (exists: {})",
			grammar_path,
			grammar_path.exists()
		);
	}

	// Try to load the grammar directly
	println!("\nAttempting to load rust grammar...");
	match load_grammar("rust") {
		Ok(grammar) => println!("Grammar loaded successfully! {:?}", grammar),
		Err(e) => println!("Grammar loading failed: {:?}", e),
	}
}

#[test]
fn test_full_highlighting_pipeline() {
	use ratatui::style::{Color, Style};

	let mut loader = LanguageLoader::new();

	// Register Rust with proper extensions
	let rust = LanguageData::new(
		"rust".to_string(),
		None,
		vec!["rs".to_string()],
		vec![],
		vec![],
		vec!["//".to_string()],
		Some(("/*".to_string(), "*/".to_string())),
		Some("rust"),
	);
	let rust_lang = loader.register(rust);

	let source = Rope::from_str("fn main() {\n    let x = 42;\n}");

	let syntax = match Syntax::new(source.slice(..), rust_lang, &loader) {
		Ok(s) => s,
		Err(e) => {
			println!("Skipping highlight test - no grammar available: {:?}", e);
			return;
		}
	};

	// Create highlight styles
	// Use actual Helix-style scope names from highlights.scm
	let styles = HighlightStyles::new(
		&[
			"keyword",
			"keyword.control",
			"keyword.function",
			"function",
			"function.method",
			"variable",
			"variable.other.member",
			"type",
			"string",
			"number",
			"operator",
		],
		|scope| match scope {
			s if s.starts_with("keyword") => Style::default().fg(Color::Red),
			s if s.starts_with("function") => Style::default().fg(Color::Blue),
			s if s.starts_with("variable") => Style::default().fg(Color::Yellow),
			s if s.starts_with("type") => Style::default().fg(Color::Green),
			s if s.starts_with("string") => Style::default().fg(Color::Magenta),
			"number" => Style::default().fg(Color::Cyan),
			"operator" => Style::default().fg(Color::White),
			_ => Style::default(),
		},
	);

	// Get highlighter for full document
	let highlighter = syntax.highlighter(source.slice(..), &loader, ..);

	// Collect all highlight spans
	let spans: Vec<_> = highlighter.collect();

	println!("Found {} highlight spans", spans.len());
	for span in &spans {
		let text = source.slice(span.start as usize..span.end as usize);
		let style = styles.style_for_highlight(span.highlight);
		println!(
			"  [{}-{}] {:?} -> {:?}",
			span.start,
			span.end,
			text.to_string(),
			style.fg
		);
	}

	// We should have at least some highlights if grammar loaded
	assert!(!spans.is_empty(), "Should produce highlight spans");
}

#[test]
fn test_language_loader_tree_house_trait() {
	// Verify LanguageLoader implements tree_house::LanguageLoader
	fn assert_language_loader<T: tree_house::LanguageLoader>() {}
	assert_language_loader::<LanguageLoader>();
}
