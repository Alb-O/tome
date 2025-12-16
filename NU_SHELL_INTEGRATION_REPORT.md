# Nushell-Powered Scratch / Command Palette Integration

This document captures the end-to-end plan to turn Tome’s scratch command buffer into a Nushell-powered command palette with ghost completions, pipe-able output, and optional embedded terminal UX. It is intended as a handoff-quality technical brief.

## Goals
- Run Nushell commands directly from the scratch buffer (default behavior), while preserving `:`-prefixed legacy palette commands.
- Provide inline/ghost completions sourced from Nushell for scratch input.
- Pipe editor data into Nushell (`$in`) and consume structured output for display or insertion.
- Keep the editor stable: incremental delivery with tests, config gates, and graceful fallbacks.

## Non-Goals (initial phase)
- Full terminal emulation in the scratch area (covered as an optional follow-up via `tui-term`).
- Shipping a full async/event-stream UX; start synchronous with light debouncing.
- Replacing existing command palette actions—only augmenting and routing through Nu by default.

## Current State (relevant hotspots)
- `crates/tome-term/src/editor.rs`
  - Scratch lifecycle: `open_scratch`, `close_scratch`, `toggle_scratch`, `execute_scratch` (lines ~240+).
  - Command dispatch: `execute_command_line` splits by whitespace and looks up `ext::find_command`.
  - Input modes and key handling already route scratch keystrokes into `InputHandler`.
  - Hooks exist for resize/focus/paste but no shell integration.
- `crates/tome-term/src/render.rs`
  - Scratch rendering uses two stacked panes plus status/message lines; no dedicated results pane.
- `crates/tome-term/src/main.rs`
  - Event loop using `termina` with paste, focus, resize support.
- Workspace dependencies: no Nushell crates present; `tome-term` depends on `termina`, `ratatui`, `tome-core` only.

## Proposed Architecture (incremental)
### 1) In-process Nushell engine wrapper (`nu_engine` module)
- Add dependency: `embed-nu = "0.9.1"` (brings `nu-*` crates transitively).
- Module responsibilities:
  - Own a singleton `Context` built once at startup (`lazy_static` or `OnceCell`).
  - Provide `run(expr: &str, input: PipelineData) -> Result<PipelineData, Error>` using `Context::eval_raw`.
  - Provide `complete(spans: &[String]) -> Result<Vec<Completion>, Error>` by evaluating a lightweight completion script or calling an `external_completer` closure inside Nu.
  - Helpers to convert Tome data to `PipelineData` (selection/buffer -> `ListStream`/`Value::String`) and to stringify or tabulate `PipelineData` for UI.
  - Config hook: environment setup (e.g., `$env.config.external_completer`) and load standard Nu config if present.

### 2) Scratch execution routing
- Default: treat scratch contents as Nushell script; feed selection as `$in` via `PipelineData::ListStream` of strings (or `Value::String` for whole-buffer piping when no selection).
- Fallback: if input starts with `:` (or another marker), strip marker and route to existing `execute_command_line` (legacy palette commands like `:write`).
- Errors: capture `ShellError`/`miette` text and surface in message line; do not crash the editor.
- Keep scratch open behavior as-is; optionally close on success based on existing `scratch_keep_open` flag.

### 3) Ghost completions in scratch insert mode
- On key input (debounced ~75–100ms) when scratch is focused and in Insert mode:
  - Build spans from current line to pass to `nu_engine::complete`.
  - Render the top suggestion as ghost text in the status/message area (non-destructive); accept with `Ctrl+Space` or `RightArrow`, cycle with `Alt+Tab`.
- If `nu_engine` unavailable or errors, silently disable completions for that cycle.

### 4) Output handling (initial)
- For now, stringify `PipelineData` using `collect_string` or `into_string` and place in the message area on success; on failure, show error string.
- Add a follow-up toggle to paste output into the buffer or open a small results pane (not in first phase to avoid layout churn).

### Optional: Embedded terminal pane via `tui-term`
- Add feature-gated support to open a `tui_term::widget::PseudoTerminal` pane bound to a PTY running `nu` for a true REPL (useful for TUI apps like `fzf`).
- Route focus/keystrokes when that pane is active; otherwise keep current editor bindings.
- This can be deferred until core Nu execution and completions land.

## API Sketches
- `nu_engine/mod.rs`
  ```rust
  pub struct NuEngine { ctx: Context }
  impl NuEngine {
      pub fn init() -> Result<&'static Self, NuError>;
      pub fn run(&self, script: &str, input: PipelineData) -> Result<PipelineData, NuError>;
      pub fn complete(&self, spans: &[String]) -> Result<Vec<CompletionItem>, NuError>;
  }
  pub struct CompletionItem { pub value: String, pub description: Option<String> }
  ```
- `Editor::execute_scratch` changes:
  - Detect `:` prefix -> legacy command path.
  - Else call `nu_engine::run(flattened, input_from_selection)`.
- `Editor` adds:
  - `fn scratch_completion_hint(&self) -> Option<String>` stored in state.
  - `fn apply_completion(&mut self)` to accept the current suggestion.

## Configuration
- New editor options (serde-backed if config exists):
  - `nu.enable` (default true if crate is available, else false with soft fallback).
  - `nu.path`/`nu.lib_dirs` (optional to point to custom Nu installations).
  - `nu.completions.debounce_ms` (default ~100ms) and `nu.completions.max_items`.
- If `embed-nu` fails to init, disable Nu features and continue with legacy behavior.

## Testing Strategy
- Unit: `nu_engine` conversions (selection -> `PipelineData`), happy/err paths of `run`, completions handling with canned scripts.
- Integration: scratch execution that returns a string; scratch execution with error; `:` command fallback; completion hint rendering/acceptance; debouncing logic.
- Snapshot/UI: ensure message line shows outputs; ensure no layout regressions when scratch closed/open.

## Risks and Mitigations
- **Dependency weight:** `embed-nu` pulls a large dependency tree; watch compile times. Mitigate via feature gate and incremental builds.
- **Linking/toolchain:** If `embed-nu` fails to build in some environments, provide a CLI flag/env var to disable and a stub engine that returns “Nu disabled”.
- **Performance:** Completions on every keypress could stutter. Mitigate with debounce and short timeouts; cache the last suggestion for the same prefix.
- **Output formatting:** `collect_string` may truncate structured data. Mitigate by trimming for message line and adding a follow-up results pane.

## Step-by-Step Implementation Plan (initial deliverable)
1) Add dependency `embed-nu = "0.9.1"` to `tome-term` (feature-gated if desired).
2) Create `crates/tome-term/src/nu_engine.rs` with `NuEngine` singleton, `run`, `complete`, and data conversion helpers; add minimal error type.
3) Wire `Editor::execute_scratch` to route to `NuEngine::run` unless the line is `:`-prefixed.
4) Add scratch completion hint state + acceptance keybinding (Ctrl+Space) and render hint in status/message line.
5) Preserve all existing scratch open/close behaviors and tests; extend tests for Nu path and fallback.
6) Leave results pane for a later PR; keep outputs in the message line initially.

## Open Questions
- Should `:` remain the only legacy marker, or also support `;`? (Default: only `:`.)
- Should we auto-load user Nu configs (may slow startup)? Default: no, or gated.
- Do we want to paste Nushell output automatically into scratch on success? (Could be a toggle.)
- Is an embedded PTY pane desired in the near term, or should it wait until core Nu path is stable?

## Notes on `tui-term` (for later)
- `tui-term/examples/nested_shell.rs` shows PTY spawn + `PseudoTerminal` widget wiring with `vt100` parsing; useful if we add a full REPL pane.
- The PTY path is heavier but unlocks true interactive tools (fzf, git TUI, etc.). Keep behind a feature flag.

---
Prepared for handoff. Next action, if approved: implement steps 1–5 above in a focused PR.
