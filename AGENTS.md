# Tome

Kakoune-inspired modal text editor in Rust.

## Design Goals

- **Orthogonal**: No tight coupling between modules, no dependency tangling. Event emitter/reciever pattern, emitters don't know what recievers may exist. Heavily utilize `linkme`'s `distributed_slices` for hiearchically inferred compile-time imports.
- **Suckless plugin system**: Plugins are written in rust, the same language as the editor's source code, though pluigins do not need to compile/depend directly on it. This is done through C ABI typed schema. The schema should not need to be manually written/kept in sync with the source code (auto-generated types & API).
- **Heavy proc macro usage**: Keeps repetative data-oriented patterns lean and composable.

## Workspace Crates

- **tome-core**: Core editing primitives and extension system; `host` feature pulls in ropey/regex/termina/linkme for embedded use.
- **tome-term**: Terminal UI (ratatui) and CLI binary `tome`; houses kitty GUI integration tests.
- **tome-macro**: Proc-macro utilities.
- **ratatui**: Forked/hackable TUI library.

All crates live in `crates/` directory.

## Extension System (`crates/tome-core/src/ext/`)

Uses `linkme` for compile-time registration. Drop a file in, it's automatically included.

| Module         | Purpose                                              |
| -------------- | ---------------------------------------------------- |
| `actions/`     | Unified keybinding handlers returning `ActionResult` |
| `keybindings/` | Key → action mappings per mode                       |
| `commands/`    | Ex-mode commands (`:write`, `:quit`)                 |
| `hooks/`       | Event lifecycle observers                            |
| `options/`     | Typed config settings                                |
| `statusline/`  | Modular status bar segments                          |
| `filetypes/`   | File type detection                                  |
| `motions/`     | Cursor movement                                      |
| `objects/`     | Text object selection                                |

Running cargo: `nix develop -c cargo {build/test/etc}`. Kitty GUI tests: `KITTY_TESTS=1 DISPLAY=:0 nix develop -c cargo test -p tome-term --test kitty_multiselect -- --nocapture --test-threads=1`.

## Integration & GUI-Driven Testing

- Approach: keep tight red/green loops with assertions in both unit tests and kitty GUI integration tests. Write failing assertions first, then iterate fixes until GUI captures go green.
- Harness: `kitty-test-harness` (git dependency, own flake) drives the real terminal, sending key sequences and capturing screens. Defaults favor WSL/kitty (X11, software GL). Current GUI suite lives in `crates/tome-term/tests/kitty_multiselect.rs`; keep tests serial and isolated per file to avoid socket/file contention.
- Why it matters: core selection ops can pass unit tests, but the live GUI harness exposes cursor/selection drift and per-cursor insert bugs. Running against the real terminal ensures fixes match user-facing behavior.
