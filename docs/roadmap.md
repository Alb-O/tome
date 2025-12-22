# Tome Improvement Task List

## Goals (as constraints)

- [ ] Preserve **orthogonal modules** (low coupling, clear boundaries)
- [ ] Keep **suckless-ish extensibility** (simple core, optional features)
- [ ] Maintain **drop-in registration ergonomics** (e.g. `linkme`), but with explicit validation
- [ ] Support **C ABI plugins** safely and predictably
- [ ] Lean into **Rust superpowers** (proc-macros, types, property tests, invariants)

______________________________________________________________________

## Next 5 commits (highest leverage sequence)

### 1) FFI ABI correctness + contract

- [ ] Change all FFI callback fields to `Option<extern "C" fn(...)>` (not `fn(...)`)
- [ ] Add `catch_unwind` wrappers so panics never cross the FFI boundary
- [ ] Write an “FFI contract” doc:
  - [ ] Ownership rules for `TomeOwnedStr` (who allocates, who frees)
  - [ ] Encoding rules (UTF-8 guarantees? bytes allowed?)
  - [ ] String layout rules (ptr/len vs NUL-terminated)
  - [ ] Threading rules (callbacks can be called on which threads?)
  - [ ] Versioning rules (`abi_version`, `struct_size`, forward/back compat expectations)
- [ ] Add a minimal “ABI sanity” test plugin (build + load + call + free)

### 2) Typed actions (`ActionId`) instead of stringly dispatch

- [ ] Introduce `ActionId` (e.g. `u32` newtype or interned symbol)
- [ ] Keep human-facing names at the edges (config/help), map to `ActionId` in registry build
- [ ] Update input pipeline to emit `ActionId` (not `&'static str` / `String`)
- [ ] Add a validation pass that rejects duplicate action names / IDs

### 3) Action metadata + capability enforcement at registry level

- [ ] Extend action descriptors with `required_caps: &[Capability]`
- [ ] Registry build validates actions declare caps (and/or defaults)
- [ ] Runtime dispatch checks caps once (before action fn runs)
- [ ] Decide policy for missing caps:
  - [ ] Hard error
  - [ ] Graceful no-op + status message
  - [ ] Defer to plugin error handling

### 4) ChangeSet performance pass (avoid repeated `.chars().count()` hot paths)

- [ ] Store cached char length in insert ops (`Insert { text, char_len }`)
- [ ] Refactor apply/map/compose paths to reuse cached lengths
- [ ] Add microbenchmarks for large inserts and repeated compositions

### 5) Selection primary stability + property tests

- [ ] Track selection primary by index or tagged marker (not equality search)
- [ ] Make normalization preserve “true primary” deterministically
- [ ] Add property tests for:
  - [ ] normalization idempotence
  - [ ] primary stability under sorting/merge/dedup
  - [ ] merge behavior when primary is inside merged spans

______________________________________________________________________

## Registry & module boundaries (keep `linkme`, reduce implicit coupling)

- [ ] Create an explicit `RegistryBuilder` stage that consumes `linkme` slices and produces `Registry`
- [ ] Centralize validation in registry build:
  - [ ] Unique action names / IDs
  - [ ] Unique command names
  - [ ] Unique hook/event names
  - [ ] Verify all referenced actions exist
- [ ] Make the produced `Registry` immutable and passed explicitly (avoid global lookups)
- [ ] Add a debug tool:
  - [ ] Print all registered actions/commands/hooks in a deterministic order
  - [ ] Print origin/module info if available (feature-gated)
- [ ] Add compile-time-ish affordances via proc-macros:
  - [ ] `#[action]` macro emits descriptor + handler fn wrapper
  - [ ] optional `#[action(default_keys = "...")]`
  - [ ] optional `#[action(caps = "...")]`

______________________________________________________________________

## Actions & dispatch (typed core, strings at edges)

- [ ] Introduce `ActionDescriptor { id, name, required_caps, handler, help, … }`
- [ ] Add a stable mapping layer:
  - [ ] `name -> ActionId`
  - [ ] `ActionId -> descriptor`
- [ ] Update keybinding resolution to return `ActionId`
- [ ] Add a “help” view that lists actions (name + description + caps)

______________________________________________________________________

## Capabilities system (strengthen what you already have)

- [ ] Make capabilities a first-class part of action metadata
- [ ] Remove ad-hoc scattered `require_*()` checks where possible:
  - [ ] Keep them for non-action internal code paths
  - [ ] Prefer centralized pre-dispatch checks for actions
- [ ] Add tests:
  - [ ] Action requiring `Search` fails/handles gracefully when absent
  - [ ] Action with no caps works in minimal context
- [ ] Consider adding “cap providers” per mode/context:
  - [ ] Editor provides: buffer, viewport, status
  - [ ] Optional: search, LSP, clipboard, filesystem, etc.

______________________________________________________________________

## Input handling (pipeline, fewer clones, clearer semantics)

### Make input a pipeline

- [ ] Refactor input processing into stages:
  - [ ] Stage 1: parse params (count/register/extend) from key stream
  - [ ] Stage 2: resolve binding for current mode + parsed modifiers
  - [ ] Stage 3: emit dispatch `{ action_id, params }`
- [ ] Add a small `InputState` struct to hold pending params cleanly
- [ ] Ensure reset logic is centralized (no scattered `reset_params()`)

### Reduce churn in command mode

- [ ] Stop passing `String` around on each keystroke
- [ ] Mutate `Mode::Command { input }` in place
- [ ] Add tests for backspace/escape/enter behavior in command mode

### Clarify “extend” semantics

- [ ] Decide if `Shift` implies extend always, or only in selection-aware contexts
- [ ] Consider representing extend as a modifier in bindings rather than “magic”
- [ ] Add tests:
  - [ ] Uppercase binding exists: it must win
  - [ ] No uppercase binding: fallback to lowercase (if desired)
  - [ ] Extend toggles behave consistently across modes

______________________________________________________________________

## Selection model (correctness + ergonomics + performance)

- [ ] Introduce a `SelectionBuilder`:
  - [ ] Collect ranges fast
  - [ ] Normalize once at end
- [ ] Fix primary tracking:
  - [ ] Use original index or explicit flag during normalization
  - [ ] Resolve primary when merging spans deterministically
- [ ] Add property tests:
  - [ ] Normalization idempotence
  - [ ] No overlaps after normalize/merge
  - [ ] Primary remains within selection set
- [ ] Add targeted unit tests:
  - [ ] Duplicate ranges with different primaries
  - [ ] Primary inside merged span
  - [ ] Adjacent merge rules

______________________________________________________________________

## ChangeSet / transactions (performance + invariants)

### Performance tasks

- [ ] Cache char lengths for inserts once (avoid repeated `.chars().count()`)
- [ ] Avoid repeated `chars().take(n).collect()` in compose:
  - [ ] Prefer slicing strategies or storing smaller normalized inserts
- [ ] Add benchmarks:
  - [ ] Large paste insert
  - [ ] Many small edits (typing)
  - [ ] Compose-heavy path (undo/redo merges)

### Correctness tasks

- [ ] Add invariants and tests:
  - [ ] `apply(invert(doc))` round-trips
  - [ ] `compose(a, b)` equals applying `a` then `b`
  - [ ] map_pos agrees with apply for sampled points
- [ ] Add fuzz/property tests for random edit sequences

______________________________________________________________________

## C ABI plugins (make it boring and safe)

- [ ] Standardize FFI types:
  - [ ] Use `repr(C)` consistently
  - [ ] Use `extern "C"` in all callback signatures and struct fields
- [ ] Define and enforce ownership:
  - [ ] Host-allocated strings freed by host `free_str`
  - [ ] Guest-allocated strings freed by guest `free_str`
  - [ ] Document which side can call which free
- [ ] Decide plugin isolation model:
  - [ ] Single-threaded callbacks only, or
  - [ ] Thread-safe + send/sync requirements documented
- [ ] Add compatibility tests:
  - [ ] Old plugin with new host (forward compat)
  - [ ] New plugin with old host (graceful failure)
- [ ] Add a “safe host shim” layer:
  - [ ] Validates function pointers are non-null before calling
  - [ ] Validates struct_size >= expected minimum
  - [ ] Validates abi_version range

______________________________________________________________________

## Testing strategy (fast feedback + deep correctness)

### Golden/snapshot tests

- [ ] Add render snapshots for key scenarios:
  - [ ] Empty buffer
  - [ ] Wrapped lines
  - [ ] Selection highlight
  - [ ] Multiple cursors
- [ ] Add deterministic output formatting (stable ordering, stable widths)

### Property / fuzz tests

- [ ] Selection invariants (see Selection section)
- [ ] ChangeSet invariants (see ChangeSet section)
- [ ] Input pipeline invariants:
  - [ ] Params reset after action emission
  - [ ] Count parsing correctness (e.g. `0`, leading zeros policy)

### Integration harness

- [ ] Expand kitty test harness coverage:
  - [ ] Basic navigation and editing flows
  - [ ] Undo/redo flows
  - [ ] Command mode entry/exit flows
  - [ ] Plugin load + basic command invocation

______________________________________________________________________

## Optional “nice-to-have” follow-ups

- [ ] Add a `--dump-registry` CLI for introspection
- [ ] Add a `--check-config` CLI to validate user keybindings against registry
- [ ] Add docs:
  - [ ] “How to write an action” (proc-macro usage)
  - [ ] “How keybinding resolution works” (pipeline)
  - [ ] “Plugin authoring guide” (FFI contract + examples)
