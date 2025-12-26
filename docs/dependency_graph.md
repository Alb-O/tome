# Tome Architecture Plan (Current)

This document is the final architecture direction for Tome. It replaces the prior "split tome-api into multiple crates" proposal and the Phase 2 challenge notes.

## Decisions

- Keep `tome-api` as the integration/app crate. It owns `Editor`, UI, runtime services, rendering, and host extension hooks.
- Keep `tome-extensions` depending on `tome-api`. Host extensions require full `Editor` access (ticks, panels, ExtensionMap).
- Keep compile-time registries for builtins in `tome-manifest` (actions, commands, hooks, motions, options, statusline, filetypes).
- Keep core crates UI-free and convert at the UI boundary; `tome-language` is the remaining exception to clean up (see Guardrails).
- Defer any `tome-render`/`tome-runtime`/`tome-extension-api` crate split until the extension model is redesigned to avoid `Editor` coupling.

## Observed dependency hierarchy (workspace crates)

Direct workspace dependencies today (non-workspace deps omitted):

```
tome-base
tome-manifest  -> tome-base
tome-input     -> tome-base, tome-manifest
tome-stdlib    -> tome-base, tome-input, tome-manifest, tome-macro
tome-theme     -> tome-base, tome-manifest
tome-language  -> tome-base, tome-manifest (and ratatui for highlight styles)
tome-api       -> tome-base, tome-input, tome-language, tome-manifest, tome-stdlib, tome-theme
tome-extensions-> tome-api, tome-base, tome-manifest, tome-stdlib, tome-theme
tome-term      -> tome-api, tome-extensions, tome-base, tome-language, tome-manifest, tome-stdlib, tome-theme
```

This graph is already acyclic, but `tome-api` is the integration hub and `tome-extensions` depends on it because extensions need full `Editor` access.

## Final target graph

```
[tome-term] (bin)
  -> [tome-api] (integration/app: Editor + UI + runtime + render)
       -> [tome-extensions] (host extensions; depends on tome-api)
       -> [tome-stdlib] (core builtins)
       -> [tome-input]
       -> [tome-language]
       -> [tome-theme]
       -> [tome-manifest]
       -> [tome-base]
       -> [tome-macro]
```

## Guardrails

- Core crates must not take direct `ratatui` types. Use `tome-base` color/geometry/style types and convert at the UI boundary.
- `tome-language` should move to `tome-base::Style` to remove its direct `ratatui` dependency.
- `tome-base` conversions to `ratatui` remain feature-gated; UI crates opt in to keep core builds light.
- `tome-term` stays thin: CLI wiring and startup only; no editor logic.

## Preconditions for a future split

The split into `tome-render`/`tome-runtime`/`tome-extension-api` is only viable after:

- Extension hooks and panels no longer require `&mut Editor` (trait-based contexts or data-driven panels).
- Rendering reads from a render model instead of calling editor methods directly.
- Runtime services are exposed via traits instead of concrete `Editor` fields.

Until those are met, a crate split would increase coupling rather than reduce it.
