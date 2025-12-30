//! Macros for registering editor primitives at compile time.
//!
//! These macros generate static entries in [`linkme`] distributed slices,
//! enabling zero-cost registration of actions, keybindings, motions, hooks,
//! and other extensible editor components.
//!
//! # Primary Macros
//!
//! - [`action!`] - Register actions with optional keybindings and handlers
//! - [`bind!`] - Additional keybindings for existing actions
//! - [`motion!`] - Cursor/selection movement primitives
//! - [`hook!`] - Event lifecycle observers
//! - [`command!`] - Ex-mode commands (`:write`, `:quit`)
//!
//! # Secondary Macros
//!
//! - [`option!`] - Configuration options
//! - [`text_object!`] - Text object selection (`iw`, `a"`, etc.)
//! - [`statusline_segment!`] - Statusline segment definitions
//!
//! Note: Language definitions are loaded at runtime from `languages.kdl`.

#[doc(hidden)]
#[macro_export]
macro_rules! __opt {
	({$val:expr}, $default:expr) => {
		$val
	};
	(, $default:expr) => {
		$default
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __opt_slice {
	({$val:expr}) => {
		$val
	};
	() => {
		&[]
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __opt_static {
	({$val:expr}) => {
		Some($val)
	};
	() => {
		None
	};
}

/// Registers a configuration option in the [`OPTIONS`](crate::options::OPTIONS) slice.
#[macro_export]
macro_rules! option {
	($name:ident, $type:ident, $default:expr, $scope:ident, $desc:expr) => {
		paste::paste! {
			#[allow(non_upper_case_globals)]
			#[linkme::distributed_slice($crate::options::OPTIONS)]
			static [<OPT_ $name>]: $crate::options::OptionDef = $crate::options::OptionDef {
				id: concat!(env!("CARGO_PKG_NAME"), "::", stringify!($name)),
				name: stringify!($name),
				description: $desc,
				value_type: $crate::options::OptionType::$type,
				default: || $crate::options::OptionValue::$type($default),
				scope: $crate::options::OptionScope::$scope,
				source: $crate::RegistrySource::Crate(env!("CARGO_PKG_NAME")),
			};
		}
	};
}

/// Registers an ex-mode command in the [`COMMANDS`](crate::COMMANDS) slice.
#[macro_export]
macro_rules! command {
	($name:ident, {
		$(aliases: $aliases:expr,)?
		description: $desc:expr
		$(, priority: $priority:expr)?
		$(, caps: $caps:expr)?
		$(, flags: $flags:expr)?
		$(, source: $source:expr)?
		$(,)?
	}, handler: $handler:expr) => {
		paste::paste! {
			#[allow(non_upper_case_globals)]
			#[linkme::distributed_slice($crate::COMMANDS)]
			static [<CMD_ $name>]: $crate::CommandDef = $crate::CommandDef {
				id: concat!(env!("CARGO_PKG_NAME"), "::", stringify!($name)),
				name: stringify!($name),
				aliases: $crate::__opt_slice!($({$aliases})?),
				description: $desc,
				handler: $handler,
				user_data: None,
				priority: $crate::__opt!($({$priority})?, 0),
				source: $crate::__opt!($({$source})?, $crate::RegistrySource::Crate(env!("CARGO_PKG_NAME"))),
				required_caps: $crate::__opt_slice!($({$caps})?),
				flags: $crate::__opt!($({$flags})?, $crate::flags::NONE),
			};
		}
	};
}

/// Registers an action in the [`ACTIONS`](crate::ACTIONS) slice.
///
/// # Forms
///
/// ```ignore
/// // Basic action with handler function
/// action!(name, { description: "..." }, handler: my_handler);
///
/// // Action with inline closure
/// action!(name, { description: "..." }, |ctx| { ... });
///
/// // Action with KDL keybindings
/// action!(name, { description: "...", bindings: r#"normal "x""# }, |ctx| { ... });
///
/// // Window-style action with key, mode, result, and buffer ops handler
/// action!(name, {
///     description: "...",
///     key: Key::char('s'),
///     mode: Window,
///     result: SplitHorizontal,
///     handler_slice: RESULT_SPLIT_HORIZONTAL_HANDLERS,
/// }, |ops| ops.split_horizontal());
/// ```
#[macro_export]
macro_rules! action {
	($name:ident, {
		description: $desc:expr,
		key: $key:expr,
		mode: $mode:ident,
		result: $result:ident,
		handler_slice: $slice:ident
		$(,)?
	}, |$ops:ident| $body:expr) => {
		paste::paste! {
			$crate::action!($name, { description: $desc },
				|_ctx| $crate::actions::ActionResult::$result);

			#[::linkme::distributed_slice($crate::keybindings::[<KEYBINDINGS_ $mode:upper>])]
			static [<KB_ $name:upper>]: $crate::keybindings::KeyBindingDef =
				$crate::keybindings::KeyBindingDef {
					mode: $crate::keybindings::BindingMode::$mode,
					key: $key,
					action: stringify!($name),
					priority: 100,
				};

			#[::linkme::distributed_slice($crate::actions::$slice)]
			static [<HANDLE_ $name:upper>]: $crate::editor_ctx::ResultHandler =
				$crate::editor_ctx::ResultHandler {
					name: stringify!($name),
					handle: |r, ctx, _| {
						use $crate::editor_ctx::MessageAccess;
						if matches!(r, $crate::actions::ActionResult::$result) {
							if let Some($ops) = ctx.buffer_ops() {
								$body;
							} else {
								ctx.notify("warning", "Buffer operations not available");
							}
						}
						$crate::editor_ctx::HandleOutcome::Handled
					},
				};
		}
	};

	($name:ident, {
		$(aliases: $aliases:expr,)?
		description: $desc:expr,
		bindings: $kdl:literal
		$(, priority: $priority:expr)?
		$(, caps: $caps:expr)?
		$(, flags: $flags:expr)?
		$(,)?
	}, |$ctx:ident| $body:expr) => {
		paste::paste! {
			#[allow(unused_variables)]
			fn [<handler_ $name>]($ctx: &$crate::actions::ActionContext) -> $crate::actions::ActionResult {
				$body
			}

			$crate::action!($name, {
				$(aliases: $aliases,)?
				description: $desc,
				bindings: $kdl
				$(, priority: $priority)?
				$(, caps: $caps)?
				$(, flags: $flags)?
			}, handler: [<handler_ $name>]);
		}
	};

	($name:ident, {
		$(aliases: $aliases:expr,)?
		description: $desc:expr,
		bindings: $kdl:literal
		$(, priority: $priority:expr)?
		$(, caps: $caps:expr)?
		$(, flags: $flags:expr)?
		$(,)?
	}, handler: $handler:expr) => {
		$crate::action!($name, {
			$(aliases: $aliases,)?
			description: $desc
			$(, priority: $priority)?
			$(, caps: $caps)?
			$(, flags: $flags)?
		}, handler: $handler);
		evildoer_macro::parse_keybindings!($name, $kdl);
	};

	($name:ident, {
		$(aliases: $aliases:expr,)?
		description: $desc:expr
		$(, priority: $priority:expr)?
		$(, caps: $caps:expr)?
		$(, flags: $flags:expr)?
		$(,)?
	}, handler: $handler:expr) => {
		paste::paste! {
			#[allow(non_upper_case_globals)]
			#[linkme::distributed_slice($crate::ACTIONS)]
			static [<ACTION_ $name>]: $crate::actions::ActionDef = $crate::actions::ActionDef {
				id: concat!(env!("CARGO_PKG_NAME"), "::", stringify!($name)),
				name: stringify!($name),
				aliases: $crate::__opt_slice!($({$aliases})?),
				description: $desc,
				handler: $handler,
				priority: $crate::__opt!($({$priority})?, 0),
				source: $crate::RegistrySource::Crate(env!("CARGO_PKG_NAME")),
				required_caps: $crate::__opt_slice!($({$caps})?),
				flags: $crate::__opt!($({$flags})?, $crate::flags::NONE),
			};
		}
	};

	($name:ident, {
		$(aliases: $aliases:expr,)?
		description: $desc:expr
		$(, priority: $priority:expr)?
		$(, caps: $caps:expr)?
		$(, flags: $flags:expr)?
		$(,)?
	}, |$ctx:ident| $body:expr) => {
		paste::paste! {
			#[allow(unused_variables)]
			fn [<handler_ $name>]($ctx: &$crate::actions::ActionContext) -> $crate::actions::ActionResult {
				$body
			}
			$crate::action!($name, {
				$(aliases: $aliases,)?
				description: $desc
				$(, priority: $priority)?
				$(, caps: $caps)?
				$(, flags: $flags)?
			}, handler: [<handler_ $name>]);
		}
	};
}

/// Register additional keybindings for an existing action.
///
/// # Example
///
/// ```ignore
/// bind!(scroll_down, r#"view "j""#);
/// ```
#[macro_export]
macro_rules! bind {
	($action:ident, $kdl:literal) => {
		evildoer_macro::parse_keybindings!($action, $kdl);
	};
}

/// Define a hook and register it in the [`HOOKS`](crate::hooks::HOOKS) slice.
///
/// # Example
///
/// ```ignore
/// hook!(log_open, BufferOpen, 100, "Log buffer opens", |ctx| {
///     if let HookContext::BufferOpen { path, .. } = ctx {
///         log::info!("Opened: {}", path.display());
///     }
/// });
/// ```
#[macro_export]
macro_rules! hook {
	($name:ident, $event:ident, $priority:expr, $desc:expr, |$ctx:ident| $body:expr) => {
		paste::paste! {
			#[allow(clippy::unused_unit)]
			fn [<hook_handler_ $name>]($ctx: &$crate::hooks::HookContext) -> $crate::hooks::HookAction {
				let result = { $body };
				::core::convert::Into::into(result)
			}

			#[allow(non_upper_case_globals)]
			#[linkme::distributed_slice($crate::hooks::HOOKS)]
			static [<HOOK_ $name>]: $crate::hooks::HookDef = $crate::hooks::HookDef {
				id: concat!(env!("CARGO_PKG_NAME"), "::", stringify!($name)),
				name: stringify!($name),
				event: $crate::hooks::HookEvent::$event,
				description: $desc,
				priority: $priority,
				handler: [<hook_handler_ $name>],
				source: $crate::RegistrySource::Crate(env!("CARGO_PKG_NAME")),
			};
		}
	};
}

/// Registers a text object in the [`TEXT_OBJECTS`](crate::TEXT_OBJECTS) slice.
#[macro_export]
macro_rules! text_object {
	($name:ident, {
		trigger: $trigger:expr,
		$(alt_triggers: $alt_triggers:expr,)?
		$(aliases: $aliases:expr,)?
		description: $desc:expr
		$(, priority: $priority:expr)?
		$(, caps: $caps:expr)?
		$(, flags: $flags:expr)?
		$(, source: $source:expr)?
		$(,)?
	}, {
		inner: $inner:expr,
		around: $around:expr $(,)?
	}) => {
		paste::paste! {
			#[allow(non_upper_case_globals)]
			#[linkme::distributed_slice($crate::TEXT_OBJECTS)]
			static [<OBJ_ $name>]: $crate::TextObjectDef = $crate::TextObjectDef {
				id: concat!(env!("CARGO_PKG_NAME"), "::", stringify!($name)),
				name: stringify!($name),
				aliases: $crate::__opt_slice!($({$aliases})?),
				trigger: $trigger,
				alt_triggers: $crate::__opt_slice!($({$alt_triggers})?),
				description: $desc,
				inner: $inner,
				around: $around,
				priority: $crate::__opt!($({$priority})?, 0),
				source: $crate::__opt!($({$source})?, $crate::RegistrySource::Crate(env!("CARGO_PKG_NAME"))),
				required_caps: $crate::__opt_slice!($({$caps})?),
				flags: $crate::__opt!($({$flags})?, $crate::flags::NONE),
			};
		}
	};
}

/// Registers a motion primitive in the [`MOTIONS`](crate::MOTIONS) slice.
#[macro_export]
macro_rules! motion {
	($name:ident, {
		$(aliases: $aliases:expr,)?
		description: $desc:expr
		$(, priority: $priority:expr)?
		$(, caps: $caps:expr)?
		$(, flags: $flags:expr)?
		$(, source: $source:expr)?
		$(,)?
	}, |$text:ident, $range:ident, $count:ident, $extend:ident| $body:expr) => {
		paste::paste! {
			#[allow(unused_variables, non_snake_case)]
			fn [<motion_handler_ $name>](
				$text: ropey::RopeSlice,
				$range: $crate::Range,
				$count: usize,
				$extend: bool,
			) -> $crate::Range {
				$body
			}

			#[allow(non_upper_case_globals)]
			#[linkme::distributed_slice($crate::MOTIONS)]
			static [<MOTION_ $name>]: $crate::MotionDef = $crate::MotionDef {
				id: concat!(env!("CARGO_PKG_NAME"), "::", stringify!($name)),
				name: stringify!($name),
				aliases: $crate::__opt_slice!($({$aliases})?),
				description: $desc,
				handler: [<motion_handler_ $name>],
				priority: $crate::__opt!($({$priority})?, 0),
				source: $crate::__opt!($({$source})?, $crate::RegistrySource::Crate(env!("CARGO_PKG_NAME"))),
				required_caps: $crate::__opt_slice!($({$caps})?),
				flags: $crate::__opt!($({$flags})?, $crate::flags::NONE),
			};
		}
	};
}

/// Registers a statusline segment in the [`STATUSLINE_SEGMENTS`](crate::STATUSLINE_SEGMENTS) slice.
#[macro_export]
macro_rules! statusline_segment {
	($static_name:ident, $name:expr, $position:expr, $priority:expr, $enabled:expr, $render:expr) => {
		#[::linkme::distributed_slice($crate::STATUSLINE_SEGMENTS)]
		static $static_name: $crate::StatuslineSegmentDef = $crate::StatuslineSegmentDef {
			id: $name,
			name: $name,
			position: $position,
			priority: $priority,
			default_enabled: $enabled,
			render: $render,
			source: $crate::RegistrySource::Crate(env!("CARGO_PKG_NAME")),
		};
	};
}

/// Registers a handler for an [`ActionResult`](crate::ActionResult) variant.
#[macro_export]
macro_rules! result_handler {
	($slice:ident, $static_name:ident, $name:literal, $body:expr) => {
		#[::linkme::distributed_slice($crate::actions::$slice)]
		static $static_name: $crate::editor_ctx::ResultHandler =
			$crate::editor_ctx::ResultHandler {
				name: $name,
				handle: $body,
			};
	};
}

/// Registers a panel type in the [`PANELS`](crate::panels::PANELS) slice.
///
/// Panels are toggleable split views (terminals, debug logs, file trees, etc.)
/// that integrate with the editor's layer system.
///
/// # Example
///
/// ```ignore
/// // Panel definition with inline factory
/// panel!(terminal, {
///     description: "Embedded terminal emulator",
///     mode_name: "TERMINAL",
///     layer: 1,
///     sticky: true,
///     factory: || Box::new(TerminalBuffer::new()),
/// });
///
/// // Panel definition without factory (factory registered elsewhere)
/// panel!(debug, {
///     description: "Debug log viewer",
///     mode_name: "DEBUG",
///     layer: 2,
/// });
/// ```
///
/// # Fields
///
/// - `description` (required): Human-readable description
/// - `mode_name` (required): Status bar mode text when focused (e.g., "DEBUG")
/// - `layer` (required): Layer index for docking (0 = base, higher overlays lower)
/// - `singleton` (optional): Only one instance allowed (default: true)
/// - `sticky` (optional): Resist losing focus on mouse hover (default: false)
/// - `priority` (optional): Priority within layer (default: 0)
/// - `factory` (optional): Factory function `fn() -> Box<dyn Any + Send>`
#[macro_export]
macro_rules! panel {
	($name:ident, {
		description: $desc:expr,
		mode_name: $mode_name:expr,
		layer: $layer:expr
		$(, singleton: $singleton:expr)?
		$(, sticky: $sticky:expr)?
		$(, priority: $priority:expr)?
		, factory: $factory:expr
		$(,)?
	}) => {
		paste::paste! {
			#[allow(non_upper_case_globals)]
			#[linkme::distributed_slice($crate::panels::PANELS)]
			static [<PANEL_ $name:upper>]: $crate::panels::PanelDef = $crate::panels::PanelDef {
				id: concat!(env!("CARGO_PKG_NAME"), "::", stringify!($name)),
				name: stringify!($name),
				description: $desc,
				mode_name: $mode_name,
				layer: $layer,
				priority: $crate::__opt!($({$priority})?, 0),
				source: $crate::RegistrySource::Crate(env!("CARGO_PKG_NAME")),
				singleton: $crate::__opt!($({$singleton})?, true),
				sticky: $crate::__opt!($({$sticky})?, false),
			};

			#[allow(non_upper_case_globals)]
			#[linkme::distributed_slice($crate::panels::PANEL_FACTORIES)]
			static [<PANEL_FACTORY_ $name:upper>]: $crate::panels::PanelFactoryDef =
				$crate::panels::PanelFactoryDef {
					name: stringify!($name),
					factory: $factory,
				};
		}
	};

	($name:ident, {
		description: $desc:expr,
		mode_name: $mode_name:expr,
		layer: $layer:expr
		$(, singleton: $singleton:expr)?
		$(, sticky: $sticky:expr)?
		$(, priority: $priority:expr)?
		$(,)?
	}) => {
		paste::paste! {
			#[allow(non_upper_case_globals)]
			#[linkme::distributed_slice($crate::panels::PANELS)]
			static [<PANEL_ $name:upper>]: $crate::panels::PanelDef = $crate::panels::PanelDef {
				id: concat!(env!("CARGO_PKG_NAME"), "::", stringify!($name)),
				name: stringify!($name),
				description: $desc,
				mode_name: $mode_name,
				layer: $layer,
				priority: $crate::__opt!($({$priority})?, 0),
				source: $crate::RegistrySource::Crate(env!("CARGO_PKG_NAME")),
				singleton: $crate::__opt!($({$singleton})?, true),
				sticky: $crate::__opt!($({$sticky})?, false),
			};
		}
	};
}

pub use crate::{
	__opt, __opt_slice, __opt_static, action, bind, command, hook, motion, option, panel,
	result_handler, statusline_segment, text_object,
};
