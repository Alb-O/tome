//! Registry infrastructure using compile-time distributed slices.
//!
//! This crate bridges `evildoer-registry` types to the editor's infrastructure,
//! providing [`RegistryMetadata`] trait implementations, [`ActionId`] for dispatch,
//! and [`KeymapRegistry`] for trie-based keybinding lookup.
//!
//! Registry definitions live in `evildoer-registry`. This crate provides the glue
//! layer: re-exports for backward compatibility, `RegistryMetadata` impls, and
//! `ActionId` resolution infrastructure.

pub use evildoer_base::range::CharIdx;
pub use evildoer_base::{Range, Selection};

mod registry_impls;

pub use evildoer_registry::{
	bracket_pair_object, motion, option, statusline_segment, symmetric_text_object, text_object,
	Capability, RegistrySource,
};

pub mod actions;
pub mod completion;
pub mod editor_ctx;
pub mod index;
pub mod keymap_registry;
pub mod macros;
pub mod terminal_config;

/// Theme completion source.
pub mod theme {
	use evildoer_registry::themes::{runtime_themes, ThemeVariant, THEMES};

	use super::completion::{
		CompletionContext, CompletionItem, CompletionKind, CompletionResult, CompletionSource,
		PROMPT_COMMAND,
	};

	/// Completion source for theme names.
	pub struct ThemeSource;

	impl CompletionSource for ThemeSource {
		fn complete(&self, ctx: &CompletionContext) -> CompletionResult {
			if ctx.prompt != PROMPT_COMMAND {
				return CompletionResult::empty();
			}

			let parts: Vec<&str> = ctx.input.split_whitespace().collect();
			if !matches!(parts.first(), Some(&"theme") | Some(&"colorscheme")) {
				return CompletionResult::empty();
			}

			let prefix = parts.get(1).copied().unwrap_or("");
			if parts.len() == 1 && !ctx.input.ends_with(' ') {
				return CompletionResult::empty();
			}

			let cmd_name = parts.first().unwrap();
			let arg_start = cmd_name.len() + 1;

			let mut items: Vec<_> = runtime_themes()
				.iter()
				.copied()
				.chain(THEMES.iter())
				.filter(|t| {
					t.name.starts_with(prefix) || t.aliases.iter().any(|a| a.starts_with(prefix))
				})
				.map(|t| CompletionItem {
					label: t.name.to_string(),
					insert_text: t.name.to_string(),
					detail: Some(format!(
						"{} theme",
						match t.variant {
							ThemeVariant::Dark => "dark",
							ThemeVariant::Light => "light",
						}
					)),
					filter_text: None,
					kind: CompletionKind::Theme,
				})
				.collect();

			items.dedup_by(|a, b| a.label == b.label);
			CompletionResult::new(arg_start, items)
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActionId(pub u32);

impl ActionId {
	/// Represents an invalid action ID.
	pub const INVALID: ActionId = ActionId(u32::MAX);

	/// Returns true if this action ID is valid.
	#[inline]
	pub fn is_valid(self) -> bool {
		self != Self::INVALID
	}

	/// Returns the underlying u32 value.
	#[inline]
	pub fn as_u32(self) -> u32 {
		self.0
	}
}

impl std::fmt::Display for ActionId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if *self == Self::INVALID {
			write!(f, "ActionId(INVALID)")
		} else {
			write!(f, "ActionId({})", self.0)
		}
	}
}

/// Common metadata for all registry item types.
pub trait RegistryMetadata {
	fn id(&self) -> &'static str;
	fn name(&self) -> &'static str;
	fn priority(&self) -> i16;
	fn source(&self) -> RegistrySource;
}

pub use actions::{
	cursor_motion, dispatch_result, insert_with_motion, selection_motion, ActionArgs,
	ActionContext, ActionDef, ActionHandler, ActionMode, ActionResult, EditAction,
	ObjectSelectionKind, PendingAction, PendingKind, ScrollAmount, ScrollDir, VisualDirection,
};
pub use completion::{CompletionContext, CompletionItem, CompletionKind, CompletionSource};
pub use editor_ctx::{EditorCapabilities, EditorContext, EditorOps, HandleOutcome};
pub use evildoer_base::Mode;
pub use evildoer_registry::actions::ACTIONS;
pub use evildoer_registry::commands::{
	flags, CommandContext, CommandDef, CommandError, CommandOutcome, CommandResult, COMMANDS,
};
pub use evildoer_registry::hooks::{
	all_hooks, emit as emit_hook, emit_mutable as emit_mutable_hook, emit_sync as emit_hook_sync,
	emit_sync_with as emit_hook_sync_with, find_hooks, BoxFuture as HookBoxFuture, HookAction,
	HookContext, HookDef, HookEvent, HookEventData, HookHandler, HookMutability, HookResult,
	HookScheduler, MutableHookContext, OwnedHookContext, HOOKS,
};
pub use evildoer_registry::notifications::{
	find_notification_type, Animation, AutoDismiss, Level, NotificationTypeDef, Timing,
	NOTIFICATION_TYPES,
};
pub use evildoer_registry::options::{OptionDef, OptionScope, OptionType, OptionValue, OPTIONS};
pub use evildoer_registry::panels::{
	all_panels, find_factory, find_panel, find_panel_by_id, panel_kind_index, PanelDef,
	PanelFactory, PanelFactoryDef, PanelId, SplitAttrs, SplitBuffer, SplitCell, SplitColor,
	SplitCursor, SplitCursorStyle, SplitDockPreference, SplitEventResult, SplitKey, SplitKeyCode,
	SplitModifiers, SplitMouse, SplitMouseAction, SplitMouseButton, SplitSize, PANELS,
	PANEL_FACTORIES,
};
pub use evildoer_registry::statusline::{
	all_segments, find_segment, render_position, segments_for_position, RenderedSegment,
	SegmentPosition, SegmentStyle, StatuslineContext, StatuslineSegmentDef, STATUSLINE_SEGMENTS,
};
pub use evildoer_registry::text_objects::{TextObjectDef, TextObjectHandler, TEXT_OBJECTS};
pub use evildoer_registry::{BindingMode, KeyBindingDef, MotionDef, KEYBINDINGS, MOTIONS};
pub use index::{
	all_actions, all_commands, all_motions, all_text_objects, find_action, find_action_by_id,
	find_command, find_motion, find_text_object_by_trigger, resolve_action_id,
};
pub use keymap_registry::{get_keymap_registry, BindingEntry, KeymapRegistry, LookupResult};
pub use terminal_config::{TerminalConfig, TerminalSequence};
pub use theme::ThemeSource;
