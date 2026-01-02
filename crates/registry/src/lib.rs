//! Registry-first organization for Evildoer editor extensions.
//!
//! This crate aggregates all registry sub-crates. Depend on this crate to get
//! access to all registries, rather than depending on individual registry crates.
//!
//! # Sub-crates
//!
//! - [`menus`] - Menu bar groups and items
//! - [`motions`] - Cursor movement primitives
//! - [`options`] - Configuration options
//! - [`notifications`] - Notification types
//! - [`commands`] - Ex-mode command definitions
//! - [`actions`] - Action definitions and handlers
//! - [`panels`] - Panel definitions and split buffers
//! - [`hooks`] - Event lifecycle observers
//! - [`statusline`] - Statusline segment definitions
//! - [`text_objects`] - Text object selection (inner/around)
//!
//! # Adding a New Registry
//!
//! 1. Create `crates/registry/{name}/` with Cargo.toml and src/
//! 2. Add to root `Cargo.toml` members and workspace.dependencies
//! 3. Add dependency and re-export here

// Re-export commonly used items at the crate root for convenience
pub use actions::editor_ctx::{
	CommandQueueAccess, CursorAccess, EditAccess, EditorCapabilities, EditorContext, EditorOps,
	FileOpsAccess, FocusOps, HandleOutcome, JumpAccess, MacroAccess, MessageAccess, PanelOps,
	ResultHandler, SearchAccess, SelectionAccess, SplitOps, TextAccess, ThemeAccess, UndoAccess,
};
pub use actions::{
	action, dispatch_result, result_extension_handler, result_handler, ActionArgs, ActionContext,
	ActionDef, ActionHandler, ActionMode, ActionResult, BindingMode, EditAction, KeyBindingDef,
	Mode, ObjectSelectionKind, PendingAction, PendingKind, ScrollAmount, ScrollDir,
	VisualDirection, ACTIONS, KEYBINDINGS, RESULT_BUFFER_NEXT_HANDLERS,
	RESULT_BUFFER_PREV_HANDLERS, RESULT_CLOSE_OTHER_BUFFERS_HANDLERS, RESULT_CLOSE_SPLIT_HANDLERS,
	RESULT_COMMAND_HANDLERS, RESULT_CURSOR_MOVE_HANDLERS, RESULT_EDIT_HANDLERS,
	RESULT_ERROR_HANDLERS, RESULT_EXTENSION_HANDLERS, RESULT_FOCUS_DOWN_HANDLERS,
	RESULT_FOCUS_LEFT_HANDLERS, RESULT_FOCUS_RIGHT_HANDLERS, RESULT_FOCUS_UP_HANDLERS,
	RESULT_FORCE_REDRAW_HANDLERS, RESULT_INSERT_WITH_MOTION_HANDLERS, RESULT_MODE_CHANGE_HANDLERS,
	RESULT_MOTION_HANDLERS, RESULT_OK_HANDLERS, RESULT_PENDING_HANDLERS, RESULT_QUIT_HANDLERS,
	RESULT_SEARCH_NEXT_HANDLERS, RESULT_SEARCH_PREV_HANDLERS, RESULT_SPLIT_HORIZONTAL_HANDLERS,
	RESULT_SPLIT_VERTICAL_HANDLERS, RESULT_TOGGLE_PANEL_HANDLERS,
	RESULT_USE_SELECTION_SEARCH_HANDLERS,
};
pub use commands::{
	all_commands, command, find_command, CommandContext, CommandDef, CommandEditorOps,
	CommandError, CommandHandler, CommandOutcome, CommandResult, COMMANDS,
};
pub use evildoer_registry_options::option;
pub use hooks::{
	all_hooks, async_hook, emit, emit_mutable, emit_sync, emit_sync_with, find_hooks, hook, Bool,
	BoxFuture, HookAction, HookContext, HookDef, HookEvent, HookEventData, HookHandler,
	HookMutability, HookResult, HookScheduler, MutableHookContext, OptionViewId, OwnedHookContext,
	SplitDirection, Str, ViewId, HOOKS,
};
pub use menus::{menu_group, menu_item, MenuGroupDef, MenuItemDef, MENU_GROUPS, MENU_ITEMS};
// Re-export shared types (these are duplicated across registries, pick one source)
pub use motions::{flags, Capability, RegistrySource};
pub use motions::{motion, movement, MotionDef, MotionHandler, MOTIONS};
pub use notifications::{
	find_notification_type, Anchor, Animation, AnimationPhase, AutoDismiss, Level,
	NotificationError, NotificationTypeDef, Overflow, SizeConstraint, SlideDirection, Timing,
	NOTIFICATION_TYPES,
};
pub use panels::{
	all_panels, find_factory, find_panel, find_panel_by_id, panel, panel_kind_index, PanelDef,
	PanelFactory, PanelFactoryDef, PanelId, SplitAttrs, SplitBuffer, SplitCell, SplitColor,
	SplitCursor, SplitCursorStyle, SplitDockPreference, SplitEventResult, SplitKey, SplitKeyCode,
	SplitModifiers, SplitMouse, SplitMouseAction, SplitMouseButton, SplitSize, PANELS,
	PANEL_FACTORIES,
};
pub use statusline::{
	all_segments, find_segment, render_position, segments_for_position, statusline_segment,
	RenderedSegment, SegmentPosition, SegmentStyle, StatuslineContext, StatuslineSegmentDef,
	STATUSLINE_SEGMENTS,
};
pub use text_objects::{
	bracket_pair_object, symmetric_text_object, text_object, TextObjectDef, TextObjectHandler,
	TEXT_OBJECTS,
};
pub use {
	evildoer_registry_actions as actions, evildoer_registry_commands as commands,
	evildoer_registry_hooks as hooks, evildoer_registry_menus as menus,
	evildoer_registry_motions as motions, evildoer_registry_notifications as notifications,
	evildoer_registry_options as options, evildoer_registry_panels as panels,
	evildoer_registry_statusline as statusline, evildoer_registry_text_objects as text_objects,
};
