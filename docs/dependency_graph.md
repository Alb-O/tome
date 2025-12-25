# Tome Dependency Graph (Unstable)

This document proposes a new crate layout and dependency graph for Tome. It assumes compile-time extensions only and a suckless-ish philosophy: small, explicit layers, no dynamic plugins, and no stable SDK promises.

## Intentions

- Compile-time extensions only. No dynamic loading, no plugin ABI.
- Keep the dependency graph acyclic and layered.
- Push heavy dependencies (tokio, pty, ratatui) to leaf crates.
- Fast compiles by default; opt into host features explicitly.
- Unstable by design: crate boundaries can move and APIs can break.

## Non-goals

- A stable public SDK for third-party crates.
- Runtime plugin discovery or hot reload.
- Hosting multiple UI backends in core crates.

## Target layering rules

These rules are the guardrails for the new graph:

1. Core crates never depend on UI, runtime, or OS-specific crates.
1. UI crates never depend on terminal or OS crates directly; they depend on a host layer.
1. Tokio, agentfs, and IPC are confined to a runtime crate.
1. ratatui, crossterm, and terminal drawing are confined to UI or binary crates.
1. Registries and extension traits live in a dedicated extension API crate.

## Target crate graph (v2)

The intended dependency direction is shown below. Arrows point from higher to lower layers.

```
[tome-term] (bin)
  -> [tome-app] (integration)
       -> [tome-ui] (ratatui, render adapter)
       -> [tome-runtime] (tokio, pty, ipc, agentfs)
       -> [tome-extensions] (host extensions)
       -> [tome-stdlib] (core builtins)
       -> [tome-render] (layout, viewport, render model)
       -> [tome-extension-api] (registries + traits)
       -> [tome-input]
       -> [tome-language]
       -> [tome-theme]
       -> [tome-manifest-core]
       -> [tome-core]
       -> [tome-macro]
```

## What each crate contains

- tome-core: text model, selections, transactions, history, ranges, graphemes. No IO.
- tome-manifest-core: extension metadata, registries, and naming. Serde-only.
- tome-input: key state machine and input modes. No UI.
- tome-language: language metadata, queries, parsing support. No UI.
- tome-theme: color and style model, no ratatui types.
- tome-macro: proc-macros for static registration.
- tome-extension-api: traits + registration for compile-time extensions.
- tome-stdlib: builtins (actions, commands, motions), no UI or runtime.
- tome-render: render model (layout, viewport, line wrapping), no terminal backend.
- tome-runtime: async jobs, file IO, IPC, pty. No UI.
- tome-ui: TUI adapter over render model (ratatui, crossterm).
- tome-extensions: host extensions (LSP, panels, agentfs), compiled in.
- tome-app: thin integration crate that wires runtime, render, stdlib, and extensions.
- tome-term: binary crate for terminal frontend.

## Mapping from current crates

This is a suggested migration target for existing code:

- crates/base -> tome-core (rename or split)
- crates/manifest -> tome-manifest-core (strip UI deps)
- crates/input -> tome-input (mostly unchanged)
- crates/language -> tome-language (mostly unchanged)
- crates/theme -> tome-theme (remove ratatui dependency)
- crates/macro -> tome-macro (unchanged)
- crates/stdlib -> tome-stdlib (no host/UI deps)
- crates/api -> split into tome-render, tome-runtime, tome-app, tome-extension-api
- crates/extensions -> tome-extensions (depends on extension-api, not app)
- crates/term -> tome-term (binary)

## Compile-time extension model

- Registration stays compile-time via linkme slices.
- Extension traits live in tome-extension-api; implementations live in tome-stdlib or tome-extensions.
- No dynamic extension discovery. Everything is linked into the binary.

## Technical details and constraints

- Keep ratatui out of core. Use a render-model-only crate and an adapter in tome-ui.
- Keep tokio out of core. Runtime services are accessed via traits in extension-api.
- Replace any crossterm/termina types in base structs with abstract types.
- Avoid UI types in manifest; convert to UI types only at the UI boundary.
- Expose a minimal, unstable facade in tome-app to keep cross-crate churn localized.

## Update plan (major overhaul)

Phase 0 - Document and enforce boundaries [DONE]

- Add this document and update extension_model.md to match the new layering rules.
- Add a simple crate dependency check (manual or via cargo tree notes).

Phase 1 - Extract core + manifest [DONE]

- Move all UI and runtime types out of tome-manifest into tome-theme or tome-ui.
- Rename or split tome-base into tome-core with no terminal deps.

Completed:

- Created `tome_base::color` module with abstract Color, Modifier, Style types
- Removed ratatui from tome-manifest (now uses tome_base::color)
- Removed ratatui from tome-theme (re-exports tome_base::color)
- Added From trait impls for ratatui conversion (gated behind `ratatui` feature)
- Updated tome-api/tome-extensions to use `.into()` at UI boundary

Phase 2 - Split tome-api

- Create tome-render and move render and document viewport code from tome-api.
- Create tome-runtime and move IPC, async, and pty code from tome-api.
- Create tome-app as an integration facade over core, runtime, render, and stdlib.

Phase 3 - Extension API clean split

- Move extension registries and traits to tome-extension-api.
- Update tome-stdlib and tome-extensions to depend only on extension-api and core.

Phase 4 - UI cleanup

- Create tome-ui with the ratatui integration.
- Keep tome-term minimal, mostly a main() and startup wiring.

Phase 5 - Build time optimizations

- Set workspace default-members to leaf crates (tome-term and tome-app).
- Make host features opt-in; keep default features minimal.
- Avoid building ratatui in non-UI workflows.

## Technical notes

### Abstract color types (Phase 1)

The `tome_base::color` module provides UI-agnostic types:

```rust
// crates/base/src/color.rs
pub enum Color { Reset, Black, Red, ..., Rgb(u8,u8,u8), Indexed(u8) }
pub struct Modifier(u16);  // bitflags: BOLD, ITALIC, UNDERLINED, etc.
pub struct Style { fg: Option<Color>, bg: Option<Color>, modifiers: Modifier }
```

Conversion to ratatui is via `From` traits, gated behind the `ratatui` feature (default on):

```rust
// In crates that use ratatui
let ratatui_color: ratatui::style::Color = theme.colors.ui.bg.into();
let ratatui_style: ratatui::style::Style = abstract_style.into();
```

Pattern: Define abstract types in core, convert at UI boundary with `.into()`.

### Current dependency violations (remaining)

| Crate       | Issue                       | Fix                                     |
| ----------- | --------------------------- | --------------------------------------- |
| tome-stdlib | Has ratatui, crossterm deps | Move notification rendering to UI crate |
| tome-api    | Monolith with UI+runtime    | Split per Phase 2                       |
| tome-base   | Has termina (for Key)       | Acceptable for now, or abstract later   |

## Risks and tradeoffs

- This is intentionally unstable. Expect breaking APIs and moving types.
- Splitting crates increases surface area, but makes compile costs predictable.
- The extension API will be a moving target until the new graph settles.

## Open questions

- Do we want an explicit "tome-engine" name instead of tome-app?
- How strict should feature gating be for host extensions?
- Should theme live under manifest (pure data) or render (computed styles)?

## Current state (post Phase 1)

```
tome-manifest ─┬─> tome-base (no ratatui)
               └─> futures, linkme, ropey, serde, etc.

tome-theme ────┬─> tome-base (no ratatui)
               └─> tome-manifest

tome-api ──────┬─> ratatui (UI boundary, uses .into())
               ├─> tome-theme, tome-manifest, tome-stdlib
               └─> tokio, agentfs, etc.
```
