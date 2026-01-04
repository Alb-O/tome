# Xeno: Window Abstraction & Floating Windows

## Model Directive

Implement a `Window` abstraction that decouples view containers from buffers, enabling floating windows (command palette, pickers, previews) alongside the existing split-based layout. The first use case is a command palette that reuses the text buffer editor as its line editor.

**This is a foundational refactoring task** - the goal is to establish abstractions that support both current splits and future floating/overlay windows without breaking existing functionality.

______________________________________________________________________

## Design Philosophy

The current architecture assumes `BufferId == View`. Every text view is a buffer in a layout tree. This works for splits but breaks down for:

- **Command palette**: Needs a mini-editor with familiar controls, floating over content
- **Pickers**: File picker, symbol picker, fuzzy finder - all need floating UI with text input
- **Previews**: Hover documentation, signature help, AI suggestions
- **Multi-line REPL**: Expandable command input with history

The solution is a **Window** abstraction that:

1. Contains one or more views (initially just one buffer, but extensible)
2. Has its own focus state, position, and z-order
3. Can be floating (positioned absolutely) or docked (in the split tree)
4. Enables independent input handling per window

______________________________________________________________________

## Implementation Expectations

<mandatory_execution_requirements>

1. Implement changes incrementally, verifying each step compiles with `cargo check -p xeno-api`
2. Preserve existing split behavior exactly - all current tests must pass
3. Do not break the existing keybinding, action, or hook systems
4. Complete Phase 1-3 before moving to Phase 4+ (command palette)

Unacceptable:
- Breaking existing split navigation or focus behavior
- Changing action handler signatures without updating all call sites
- Partial implementations that leave the codebase in an inconsistent state

</mandatory_execution_requirements>

______________________________________________________________________

## Behavioral Constraints

<verbosity_and_scope_constraints>

- Follow existing registry patterns (`linkme` distributed slices, `define_events!` macro)
- Prefer minimal changes that achieve correctness
- Do not implement the command palette UI in this task - only the Window infrastructure
- If any instruction is ambiguous, choose the approach that minimizes API surface changes

</verbosity_and_scope_constraints>

<design_freedom>

- New types (`WindowId`, `Window`, `FloatingWindow`) are expected
- New hook events (`WindowCreated`, `WindowClosed`, `WindowFocusChanged`) are acceptable
- Refactoring `LayoutManager` to separate split-tree from window management is encouraged
- The focus model can be redesigned if it improves clarity

</design_freedom>

______________________________________________________________________

## Architecture Overview

### Current State

```
Editor
  BufferManager
    buffers: HashMap<BufferId, Buffer>
    focused_view: BufferId              <- GLOBAL focus
  LayoutManager
    layers: Vec<Option<Layout>>         <- Layer 0 = splits, Layer 1+ = overlays
    Layout::Single(BufferId) | Layout::Split { ... }
```

### Target State

```
Editor
  BufferManager
    buffers: HashMap<BufferId, Buffer>
  WindowManager                          <- NEW
    windows: HashMap<WindowId, Window>
    focused_window: WindowId             <- Window-level focus
    base_window: WindowId                <- The main split-tree window
    floating: Vec<WindowId>              <- Z-ordered floating windows
  Window (enum or trait)
    SplitWindow { layout: Layout, focused_buffer: BufferId }
    FloatingWindow { buffer: BufferId, rect: Rect, sticky: bool }
```

### Focus Model

```rust
/// What currently has keyboard focus.
pub enum FocusTarget {
    /// A buffer within a window.
    Buffer { window: WindowId, buffer: BufferId },
    /// A UI panel (file tree, terminal, etc).
    Panel(PanelId),
}

impl Editor {
    /// Returns the currently focused window.
    fn focused_window(&self) -> &Window;
    
    /// Returns the currently focused buffer (convenience).
    fn focused_buffer(&self) -> &Buffer;
}
```

### Key Invariants

1. **Exactly one focused target** at any time (buffer or panel)
2. **Base window always exists** - created at editor startup, never closed
3. **Floating windows are z-ordered** - topmost receives input first
4. **Buffers are shared** - a buffer can appear in multiple windows (but typically doesn't)
5. **Actions receive focus context** - know which window/buffer they operate on

______________________________________________________________________

## Type Definitions

### Core Types

```rust
// crates/api/src/window/types.rs

/// Unique identifier for a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowId(pub(crate) u64);

/// Window kinds.
pub enum Window {
    /// The base window containing the split tree.
    Base(BaseWindow),
    /// A floating window positioned over content.
    Floating(FloatingWindow),
}

/// The main editor window with split layout.
pub struct BaseWindow {
    pub layout: Layout,
    pub focused_buffer: BufferId,
}

/// A floating window with absolute positioning.
pub struct FloatingWindow {
    pub id: WindowId,
    pub buffer: BufferId,
    pub rect: Rect,
    /// If true, resists losing focus from mouse hover.
    pub sticky: bool,
    /// If true, closes when focus is lost.
    pub dismiss_on_blur: bool,
    /// Visual style (border, shadow, transparency).
    pub style: FloatingStyle,
}

/// Visual style for floating windows.
pub struct FloatingStyle {
    pub border: bool,
    pub shadow: bool,
    pub title: Option<String>,
}
```

### Focus Types

```rust
// crates/api/src/editor/focus.rs

/// Identifies what has keyboard focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusTarget {
    Buffer {
        window: WindowId,
        buffer: BufferId,
    },
    Panel(PanelId),
}

/// Reason for focus change (for hooks and debugging).
#[derive(Debug, Clone, Copy)]
pub enum FocusReason {
    /// User clicked on target.
    Click,
    /// User used keybinding (e.g., Ctrl+P for command palette).
    Keybinding,
    /// Programmatic focus (e.g., opening a new window).
    Programmatic,
    /// Mouse hover (if focus-follows-mouse enabled).
    Hover,
}
```

______________________________________________________________________

## Implementation Roadmap

### Phase 1: Introduce WindowId and WindowManager (Scaffolding)

**Objective**: Add `WindowId`, `Window`, `WindowManager` types without changing behavior.

**Files**:
- `crates/api/src/window/mod.rs` (new)
- `crates/api/src/window/types.rs` (new)
- `crates/api/src/window/manager.rs` (new)
- `crates/api/src/editor/mod.rs` (add window field)

**Tasks**:

1.1 Create `crates/api/src/window/` module with types above

1.2 Create `WindowManager` with:
```rust
pub struct WindowManager {
    next_id: u64,
    base: WindowId,
    windows: HashMap<WindowId, Window>,
    floating_order: Vec<WindowId>,  // bottom to top
}

impl WindowManager {
    pub fn new(base_layout: Layout, focused_buffer: BufferId) -> Self;
    pub fn base_window(&self) -> &BaseWindow;
    pub fn base_window_mut(&mut self) -> &mut BaseWindow;
    pub fn get(&self, id: WindowId) -> Option<&Window>;
    pub fn floating_windows(&self) -> impl Iterator<Item = (WindowId, &FloatingWindow)>;
}
```

1.3 Add `WindowManager` to `Editor`, initialize with base window containing current layout

1.4 Wire up accessors: `Editor::base_window()`, `Editor::base_window_mut()`

1.5 Verify: `cargo check -p xeno-api && cargo test -p xeno-api`

### Phase 2: Migrate Layout to BaseWindow

**Objective**: Move layout ownership from `LayoutManager` to `BaseWindow`.

**Files**:
- `crates/api/src/editor/layout/manager.rs`
- `crates/api/src/window/manager.rs`

**Tasks**:

2.1 Change `LayoutManager.layers[0]` to be owned by `BaseWindow.layout`

2.2 Update all `self.layout.layers[0]` accesses to go through `self.windows.base_window()`

2.3 Keep `LayoutManager` for separator state, drag state, hover animations (view-agnostic)

2.4 Verify: `cargo test --workspace` - all split tests must pass

### Phase 3: Refactor Focus Model

**Objective**: Replace `BufferManager.focused_view` with `FocusTarget` on Editor.

**Files**:
- `crates/api/src/editor/mod.rs`
- `crates/api/src/editor/focus.rs`
- `crates/api/src/editor/buffer_manager.rs`

**Tasks**:

3.1 Add `focus: FocusTarget` field to `Editor`

3.2 Add `Editor::focused_window()` and `Editor::focused_buffer()` methods

3.3 Deprecate `BufferManager.focused_view` - redirect to `Editor.focus`

3.4 Update `focus_view()`, `focus_direction()`, etc. to update `FocusTarget`

3.5 Emit `WindowFocusChanged` hook when focus moves between windows

3.6 Verify: All existing focus tests pass, hook is emitted correctly

### Phase 4: Floating Window Creation & Rendering

**Objective**: Implement floating window lifecycle and rendering.

**Files**:
- `crates/api/src/window/floating.rs` (new)
- `crates/api/src/render/document/mod.rs`

**Tasks**:

4.1 Add `WindowManager::create_floating()`:
```rust
pub fn create_floating(
    &mut self,
    buffer: BufferId,
    rect: Rect,
    style: FloatingStyle,
) -> WindowId;
```

4.2 Add `WindowManager::close_floating(id: WindowId)`

4.3 Modify render pipeline:
```
1. Render base window (splits) as before
2. For each floating window (bottom to top):
   - Clear rect with background
   - Render border if style.border
   - Render buffer content inside
   - Draw shadow if style.shadow
```

4.4 Update hit testing to check floating windows first (top to bottom)

4.5 Verify: Can create a floating window programmatically, renders correctly

### Phase 5: Floating Window Input Routing

**Objective**: Route keyboard and mouse input to focused floating window.

**Files**:
- `crates/api/src/editor/input/mod.rs`
- `crates/term/src/app.rs`

**Tasks**:

5.1 Mouse click on floating window focuses it and its buffer

5.2 Keyboard input goes to focused buffer (whether in base or floating)

5.3 Escape in floating window closes it (if dismiss_on_blur) or returns focus to base

5.4 Floating window with `sticky: true` resists mouse-hover focus changes

5.5 Verify: Can type in floating window, Escape closes it

### Phase 6: Hook Events for Windows

**Objective**: Add hook events for window lifecycle.

**Files**:
- `crates/registry/hooks/src/` (wherever define_events! is used)
- `crates/api/src/window/manager.rs`

**Tasks**:

6.1 Add events to `define_events!`:
```rust
WindowCreated => "window:created" {
    window_id: WindowId,
    kind: WindowKind,  // Base or Floating
},
WindowClosed => "window:closed" {
    window_id: WindowId,
},
WindowFocusChanged => "window:focus_changed" {
    window_id: WindowId,
    focused: Bool,
},
```

6.2 Emit events from `WindowManager` methods

6.3 Verify: Hooks fire correctly on window operations

______________________________________________________________________

## Command Palette (Future Task - Not This Spec)

Once Phases 1-6 are complete, the command palette can be implemented as:

1. Create a new buffer (empty, scratch)
2. Create floating window containing that buffer
3. Position centered, small height (1-3 lines initially)
4. Custom rendering overlay for suggestions/completions below input
5. On Enter: parse command, execute, close window
6. On Escape: close window, return focus to base

The key insight is that the **input line is just a buffer** with familiar keybindings. The suggestions panel is a separate overlay rendered below.

______________________________________________________________________

## Key Files Reference

| Purpose | File |
|---------|------|
| Window types | `crates/api/src/window/types.rs` (new) |
| Window manager | `crates/api/src/window/manager.rs` (new) |
| Focus types | `crates/api/src/editor/focus.rs` (modify) |
| Editor integration | `crates/api/src/editor/mod.rs` (modify) |
| Render pipeline | `crates/api/src/render/document/mod.rs` (modify) |
| Input routing | `crates/api/src/editor/input/mod.rs` (modify) |
| Layout manager | `crates/api/src/editor/layout/manager.rs` (modify) |
| Hook events | `crates/registry/hooks/src/` (add events) |

______________________________________________________________________

## Anti-Patterns

1. **Don't duplicate focus state**: Single source of truth in `Editor.focus`, not also in `BufferManager` and `WindowManager`

2. **Don't special-case floating in actions**: Actions should work on "focused buffer" regardless of whether it's in base or floating window

3. **Don't hardcode command palette**: Build generic floating window infrastructure that command palette uses

4. **Don't break split navigation**: `focus_direction()`, `split_horizontal()`, etc. must work exactly as before for base window

5. **Don't mix rendering concerns**: Floating window rendering should be composable, not interleaved with split rendering

______________________________________________________________________

## Success Criteria

Phase 1-3 (Foundation):
- [ ] `WindowId`, `Window`, `WindowManager` types exist
- [ ] `Editor.focus: FocusTarget` replaces `BufferManager.focused_view`
- [ ] `Editor::focused_buffer()` returns correct buffer for any focus state
- [ ] All existing tests pass
- [ ] No behavioral changes to user-facing functionality

Phase 4-6 (Floating Windows):
- [ ] Can create floating window with `WindowManager::create_floating()`
- [ ] Floating windows render above base window
- [ ] Hit testing correctly identifies floating windows
- [ ] Keyboard input routes to focused floating window
- [ ] Mouse click on floating window focuses it
- [ ] Escape closes floating window
- [ ] Hook events fire on window lifecycle

______________________________________________________________________

## Migration Notes

### For Action Handlers

Current:
```rust
action!(my_action, { ... }, |ctx| {
    ctx.cursor_mut().move_left();  // Operates on focused buffer
});
```

After refactoring:
```rust
// No change needed - ctx.cursor_mut() still operates on focused buffer
// The focus model change is internal to Editor
```

### For Extensions

Extensions that access `editor.buffer()` or `editor.buffers.focused_view` should continue to work because:

1. `Editor::buffer()` is updated to return `focused_buffer()` from new focus model
2. `BufferManager::focused_view` becomes a computed property delegating to `Editor.focus`

### For Hooks

New hooks are additive. Existing `ViewFocusChanged` still fires for buffer focus changes within windows. New `WindowFocusChanged` fires when focus moves between windows.

______________________________________________________________________

## Testing Strategy

### Unit Tests

- `WindowManager` creation, window lifecycle
- `FocusTarget` transitions (buffer to buffer, buffer to panel, window to window)
- Hit testing with overlapping floating windows

### Integration Tests

- Create floating window, type text, close with Escape
- Split navigation still works with floating window open
- Focus returns to correct buffer after floating window closes

### Regression Tests

- All existing split tests
- All existing focus navigation tests
- All existing action tests
