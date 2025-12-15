//! Built-in file type registrations.

use linkme::distributed_slice;

use super::{FileTypeDef, FILE_TYPES};

#[distributed_slice(FILE_TYPES)]
static FT_RUST: FileTypeDef = FileTypeDef {
    name: "rust",
    extensions: &["rs"],
    filenames: &[],
    first_line_patterns: &[],
    description: "Rust source file",
};

#[distributed_slice(FILE_TYPES)]
static FT_PYTHON: FileTypeDef = FileTypeDef {
    name: "python",
    extensions: &["py", "pyw", "pyi"],
    filenames: &[],
    first_line_patterns: &["python", "python3"],
    description: "Python source file",
};

#[distributed_slice(FILE_TYPES)]
static FT_JAVASCRIPT: FileTypeDef = FileTypeDef {
    name: "javascript",
    extensions: &["js", "mjs", "cjs"],
    filenames: &[],
    first_line_patterns: &["node"],
    description: "JavaScript source file",
};

#[distributed_slice(FILE_TYPES)]
static FT_TYPESCRIPT: FileTypeDef = FileTypeDef {
    name: "typescript",
    extensions: &["ts", "mts", "cts"],
    filenames: &[],
    first_line_patterns: &[],
    description: "TypeScript source file",
};

#[distributed_slice(FILE_TYPES)]
static FT_TSX: FileTypeDef = FileTypeDef {
    name: "tsx",
    extensions: &["tsx"],
    filenames: &[],
    first_line_patterns: &[],
    description: "TypeScript JSX file",
};

#[distributed_slice(FILE_TYPES)]
static FT_JSX: FileTypeDef = FileTypeDef {
    name: "jsx",
    extensions: &["jsx"],
    filenames: &[],
    first_line_patterns: &[],
    description: "JavaScript JSX file",
};

#[distributed_slice(FILE_TYPES)]
static FT_C: FileTypeDef = FileTypeDef {
    name: "c",
    extensions: &["c", "h"],
    filenames: &[],
    first_line_patterns: &[],
    description: "C source file",
};

#[distributed_slice(FILE_TYPES)]
static FT_CPP: FileTypeDef = FileTypeDef {
    name: "cpp",
    extensions: &["cpp", "cc", "cxx", "hpp", "hh", "hxx", "c++", "h++"],
    filenames: &[],
    first_line_patterns: &[],
    description: "C++ source file",
};

#[distributed_slice(FILE_TYPES)]
static FT_GO: FileTypeDef = FileTypeDef {
    name: "go",
    extensions: &["go"],
    filenames: &[],
    first_line_patterns: &[],
    description: "Go source file",
};

#[distributed_slice(FILE_TYPES)]
static FT_JAVA: FileTypeDef = FileTypeDef {
    name: "java",
    extensions: &["java"],
    filenames: &[],
    first_line_patterns: &[],
    description: "Java source file",
};

#[distributed_slice(FILE_TYPES)]
static FT_JSON: FileTypeDef = FileTypeDef {
    name: "json",
    extensions: &["json", "jsonc"],
    filenames: &[".prettierrc", ".eslintrc"],
    first_line_patterns: &[],
    description: "JSON file",
};

#[distributed_slice(FILE_TYPES)]
static FT_YAML: FileTypeDef = FileTypeDef {
    name: "yaml",
    extensions: &["yaml", "yml"],
    filenames: &[],
    first_line_patterns: &[],
    description: "YAML file",
};

#[distributed_slice(FILE_TYPES)]
static FT_TOML: FileTypeDef = FileTypeDef {
    name: "toml",
    extensions: &["toml"],
    filenames: &["Cargo.toml", "Pipfile"],
    first_line_patterns: &[],
    description: "TOML file",
};

#[distributed_slice(FILE_TYPES)]
static FT_MARKDOWN: FileTypeDef = FileTypeDef {
    name: "markdown",
    extensions: &["md", "markdown", "mkd"],
    filenames: &["README", "CHANGELOG"],
    first_line_patterns: &[],
    description: "Markdown file",
};

#[distributed_slice(FILE_TYPES)]
static FT_HTML: FileTypeDef = FileTypeDef {
    name: "html",
    extensions: &["html", "htm", "xhtml"],
    filenames: &[],
    first_line_patterns: &["<!DOCTYPE html", "<!doctype html"],
    description: "HTML file",
};

#[distributed_slice(FILE_TYPES)]
static FT_CSS: FileTypeDef = FileTypeDef {
    name: "css",
    extensions: &["css"],
    filenames: &[],
    first_line_patterns: &[],
    description: "CSS file",
};

#[distributed_slice(FILE_TYPES)]
static FT_NIX: FileTypeDef = FileTypeDef {
    name: "nix",
    extensions: &["nix"],
    filenames: &[],
    first_line_patterns: &[],
    description: "Nix expression",
};

#[distributed_slice(FILE_TYPES)]
static FT_SH: FileTypeDef = FileTypeDef {
    name: "sh",
    extensions: &["sh", "bash", "zsh"],
    filenames: &[".bashrc", ".zshrc", ".profile", ".bash_profile"],
    first_line_patterns: &["#!/bin/sh", "#!/bin/bash", "#!/usr/bin/env bash", "#!/bin/zsh"],
    description: "Shell script",
};

#[distributed_slice(FILE_TYPES)]
static FT_MAKEFILE: FileTypeDef = FileTypeDef {
    name: "makefile",
    extensions: &["mk"],
    filenames: &["Makefile", "makefile", "GNUmakefile"],
    first_line_patterns: &[],
    description: "Makefile",
};

#[distributed_slice(FILE_TYPES)]
static FT_GITIGNORE: FileTypeDef = FileTypeDef {
    name: "gitignore",
    extensions: &["gitignore"],
    filenames: &[".gitignore", ".gitattributes"],
    first_line_patterns: &[],
    description: "Git ignore file",
};
