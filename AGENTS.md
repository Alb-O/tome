# Tome

Kakoune-inspired modal text editor in Rust. Two crates:

- **tome-core**: Core library (input, motions, selections, extension system)
- **tome-term**: Terminal UI (ratatui, editor state, rendering)

## Extension System (`tome-core/src/ext/`)

Uses `linkme` for compile-time registration. Drop a file in, it's automatically included.

| Module | Purpose |
|--------|---------|
| `actions/` | Unified keybinding handlers returning `ActionResult` |
| `keybindings/` | Key → action mappings per mode |
| `commands/` | Ex-mode commands (`:write`, `:quit`) |
| `hooks/` | Event lifecycle observers |
| `options/` | Typed config settings |
| `statusline/` | Modular status bar segments |
| `filetypes/` | File type detection |
| `motions/` | Cursor movement |
| `objects/` | Text object selection |

## Build

```sh
cargo build    # compile
cargo test     # test
nix develop    # dev shell
```
