## Code Style

- Prefer early returns over `else`; prefer `const` over `let mut` when possible
- Use `?` for error propagation; avoid `.unwrap()` in library code
- Prefer single-word variable names where unambiguous

## Build

`cargo build` compiles the project. `cargo test` runs tests. `nix build` produces a derivation. Use `nix develop` for the dev shell.

## Architecture

Standard Rust binary+library layout. `src/main.rs` is the CLI entry point, `src/lib.rs` contains reusable library code.
