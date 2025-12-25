# Tome Extension Model (Compile-Time Only)

Tome uses a compile-time extension model with a suckless-ish philosophy: everything is linked in, no dynamic plugins, and crate boundaries are intentionally unstable.

## Principles

- Compile-time extensions only. No ABI, no dynamic loading, no hot reload.
- Keep core logic free of host or UI dependencies.
- Push heavy deps (tokio, pty, ratatui) to leaf crates.
- Accept instability: APIs and crate boundaries may move.

## 1. Core Builtins (tome-stdlib)

Core builtins define the language of the editor and are registered at compile time.

- Responsibilities:
  - Actions: high-level editor operations (delete, insert, transform).
  - Commands: ex-style commands (write, quit, etc.).
  - Motions: cursor/selection movement logic.
  - Text Objects: selection targets (word, paragraph, brackets).
  - File types: language detection helpers.
- Characteristics:
  - Stateless: operate on ActionContext/CommandContext.
  - Portable: no UI or host-specific types.
  - Static registration via linkme/distributed_slice.
- Dependencies:
  - Targets tome-manifest-core and tome-extension-api, not the app.

## 2. Host Extensions (tome-extensions)

Host extensions define the environment of the editor and are also registered at compile time.

- Responsibilities:
  - Stateful services (LSP, agentfs, background tasks).
  - UI panels and host-specific UI glue.
  - Editor lifecycle hooks (ticks, startup registration).
- Characteristics:
  - Stateful: store data in ExtensionMap.
  - Host-specific: depend on the app/runtime/UI layers.
  - Built as a separate crate to avoid circular dependencies.

## 3. Extension API Boundary (tome-extension-api)

The extension API is a small, unstable boundary that unifies registries and contexts.

- Owns the registries for actions, commands, and host extensions.
- Defines the ExtensionMap and tick hooks used by host extensions.
- Exposes context types for builtin operations.
- Remains intentionally unstable; it is not a public SDK.

## 4. Dependency Direction (Target Graph)

The target graph is layered to keep the core small and buildable without UI/runtime deps.

```
[tome-term] (bin)
  -> [tome-app] (integration)
       -> [tome-ui] (ratatui adapter)
       -> [tome-runtime] (tokio, pty, ipc)
       -> [tome-extensions]
       -> [tome-stdlib]
       -> [tome-render]
       -> [tome-extension-api]
       -> [tome-input]
       -> [tome-language]
       -> [tome-theme]
       -> [tome-manifest-core]
       -> [tome-core]
       -> [tome-macro]
```

Rules:

1. Core crates never depend on UI or runtime crates.
1. UI crates depend only on render/core, not runtime.
1. Runtime crates depend on core but not UI.
1. Extensions depend on extension-api and higher layers only.

## 5. Summary Table

| Feature      | Core Builtins                 | Host Extensions                     |
| ------------ | ----------------------------- | ----------------------------------- |
| Crate        | tome-stdlib                   | tome-extensions                     |
| API Boundary | tome-extension-api + manifest | tome-extension-api + app/runtime/UI |
| Logic Type   | Functional / pure             | Stateful / side-effectful           |
| Discovery    | linkme (compile-time)         | linkme (compile-time)               |
| Examples     | move_line_down, :quit         | LspClient, ChatPanel                |

## 6. Transitional Notes

- The current codebase still uses tome-api as a monolith. The target is to split it into tome-render, tome-runtime, tome-app, and tome-extension-api.
- This document describes the intended end state and should guide refactors and new work.

## 7. Migration Progress

### Phase 1 Complete

Abstract color/style types now live in `tome-base`:

- `tome_base::Color` - UI-agnostic color enum
- `tome_base::Modifier` - text style modifiers (bold, italic, etc.)
- `tome_base::Style` - combined fg/bg/modifiers

Crates updated:

- `tome-manifest` - no longer depends on ratatui
- `tome-theme` - no longer depends on ratatui, re-exports from tome_base

UI boundary pattern:

```rust
// In tome-api or other UI crates
Style::default().bg(theme.colors.ui.bg.into())  // .into() converts to ratatui
```

### Remaining work

- tome-stdlib still has ratatui/crossterm (notification rendering)
- tome-api needs split into render/runtime/app/extension-api
- tome-extensions should depend on extension-api, not full app
