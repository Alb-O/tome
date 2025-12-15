## Code Style

- Prefer early returns over `else`; prefer `const` over `let mut` when possible
- Use `?` for error propagation; avoid `.unwrap()` in library code
- Prefer single-word variable names where unambiguous

## Build

`cargo build` compiles the project. `cargo test` runs tests. `nix build` produces a derivation. Use `nix develop` for the dev shell.

## Architecture

Two crates:
- `tome-core`: Core library with input handling, motions, text objects, and extension system
- `tome-term`: Terminal UI using ratatui, command execution, rendering

### Extension System (`tome-core/src/ext/`)

Uses `linkme` for zero-cost compile-time registration. New extensions are automatically included by adding to the distributed slices.

- **Actions** (`actions/`): Unified command/motion abstraction with string-based dispatch
  - `ActionDef`: Registers an action by name with a handler function
  - `ActionContext`/`ActionResult`: Context and results for action handlers
  - `EditAction`: Enum for operations requiring editor state (delete, yank, scroll, etc.)
  - `PendingKind`: Describes pending input needed (find char, replace, text object)
  - Submodules: `editing.rs`, `find.rs`, `insert.rs`, `modes.rs`, `motions.rs`, `scroll.rs`, `selection_ops.rs`, `text_objects.rs`
  
- **Keybindings** (`keybindings/`): Maps keys to actions per mode
  - `KeyBindingDef`: Registers a key -> action mapping with priority
  - New bindings checked first, falling back to legacy `keymap.rs`
  - Submodules: `normal.rs`, `goto.rs`, `view.rs`

- **Hooks** (`hooks/`): Event-driven lifecycle hooks
  - `HookDef`: Immutable event observers
  - `MutableHookDef`: Hooks that can modify editor state

- **Options** (`options/`): Typed configuration settings
  - `OptionDef`: Bool/Int/String settings with scope (global/buffer)

- **Motions** (`motions/`): Cursor movement functions
- **Objects** (`objects/`): Text object selection (word, quotes, etc.)
- **Commands** (`commands/`): Ex-mode commands (`:write`, `:quit`, etc.)
- **Filetypes** (`filetypes/`): Filetype detection and settings

### Migration Status

Most normal mode operations now use the action system:
- **Migrated**: h/j/k/l movement, word motions (w/b/e/W/B/E/alt-w/alt-b/alt-e), line movement (0/^/$/alt-h/alt-l), document movement (g/G), editing (d/c/y/p/r), insert entry (i/a/I/A/o/O), find char (f/t/alt-f/alt-t), text objects (alt-i/alt-a/[/]/\{/\}), selection ops (;/,/alt-,/alt-;/alt-:/()/%/x/alt-x), undo/redo (u/U), indent (>/<), case (`/~/alt-`), join (alt-j), scrolling (Ctrl-u/d/b/f), modes (v/:)
- **Legacy**: search (/ ? n N), macros (q/Q), regex selection (s/S), marks (z/Z), pipes (|/!), repeat (. alt-.), misc (&/@/_/C/+)

### Legacy System

`keymap.rs` contains the original `Command` enum and static keymaps. The hybrid approach allows gradual migration. When a keybinding exists in both systems, the new action system takes priority.

### Pending Action System

For actions requiring character input (f/t, r, alt-i/alt-a), the action returns `ActionResult::Pending(PendingAction)` which triggers `Mode::PendingAction(PendingKind)`. When the user types a character, `InputHandler` dispatches `KeyResult::ActionWithChar` with the char argument.

