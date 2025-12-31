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
macro_rules! __hook_param_expr {
	(Option<& $inner:ty>, $value:ident) => {
		$value.as_deref()
	};
	(Option < & $inner:ty >, $value:ident) => {
		$value.as_deref()
	};
	(& $inner:ty, $value:ident) => {
		&$value
	};
	(&$inner:ty, $value:ident) => {
		&$value
	};
	($ty:ty, $value:ident) => {
		$value
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __hook_extract {
	(EditorStart, $ctx:ident $(,)?) => {
		let $crate::hooks::HookEventData::EditorStart = &$ctx.data else {
			return $crate::hooks::HookAction::Done($crate::hooks::HookResult::Continue);
		};
	};
	(EditorQuit, $ctx:ident $(,)?) => {
		let $crate::hooks::HookEventData::EditorQuit = &$ctx.data else {
			return $crate::hooks::HookAction::Done($crate::hooks::HookResult::Continue);
		};
	};
	(EditorTick, $ctx:ident $(,)?) => {
		let $crate::hooks::HookEventData::EditorTick = &$ctx.data else {
			return $crate::hooks::HookAction::Done($crate::hooks::HookResult::Continue);
		};
	};
	(FocusGained, $ctx:ident $(,)?) => {
		let $crate::hooks::HookEventData::FocusGained = &$ctx.data else {
			return $crate::hooks::HookAction::Done($crate::hooks::HookResult::Continue);
		};
	};
	(FocusLost, $ctx:ident $(,)?) => {
		let $crate::hooks::HookEventData::FocusLost = &$ctx.data else {
			return $crate::hooks::HookAction::Done($crate::hooks::HookResult::Continue);
		};
	};
	(BufferOpen, $ctx:ident, $( $param:ident : $ty:ty ),* $(,)?) => {
		let $crate::hooks::HookEventData::BufferOpen { $($param,)* .. } = &$ctx.data else {
			return $crate::hooks::HookAction::Done($crate::hooks::HookResult::Continue);
		};
		$(let $param: $ty = $param; )*
	};
	(BufferWritePre, $ctx:ident, $( $param:ident : $ty:ty ),* $(,)?) => {
		let $crate::hooks::HookEventData::BufferWritePre { $($param,)* .. } = &$ctx.data else {
			return $crate::hooks::HookAction::Done($crate::hooks::HookResult::Continue);
		};
		$(let $param: $ty = $param; )*
	};
	(BufferWrite, $ctx:ident, $( $param:ident : $ty:ty ),* $(,)?) => {
		let $crate::hooks::HookEventData::BufferWrite { $($param,)* .. } = &$ctx.data else {
			return $crate::hooks::HookAction::Done($crate::hooks::HookResult::Continue);
		};
		$(let $param: $ty = $param; )*
	};
	(BufferClose, $ctx:ident, $( $param:ident : $ty:ty ),* $(,)?) => {
		let $crate::hooks::HookEventData::BufferClose { $($param,)* .. } = &$ctx.data else {
			return $crate::hooks::HookAction::Done($crate::hooks::HookResult::Continue);
		};
		$(let $param: $ty = $param; )*
	};
	(BufferChange, $ctx:ident, $( $param:ident : $ty:ty ),* $(,)?) => {
		let $crate::hooks::HookEventData::BufferChange { $($param,)* .. } = &$ctx.data else {
			return $crate::hooks::HookAction::Done($crate::hooks::HookResult::Continue);
		};
		$(let $param: $ty = $param; )*
	};
	(ModeChange, $ctx:ident, $( $param:ident : $ty:ty ),* $(,)?) => {
		let $crate::hooks::HookEventData::ModeChange { $($param,)* .. } = &$ctx.data else {
			return $crate::hooks::HookAction::Done($crate::hooks::HookResult::Continue);
		};
		$(let $param: $ty = $param; )*
	};
	(CursorMove, $ctx:ident, $( $param:ident : $ty:ty ),* $(,)?) => {
		let $crate::hooks::HookEventData::CursorMove { $($param,)* .. } = &$ctx.data else {
			return $crate::hooks::HookAction::Done($crate::hooks::HookResult::Continue);
		};
		$(let $param: $ty = $param; )*
	};
	(SelectionChange, $ctx:ident, $( $param:ident : $ty:ty ),* $(,)?) => {
		let $crate::hooks::HookEventData::SelectionChange { $($param,)* .. } = &$ctx.data else {
			return $crate::hooks::HookAction::Done($crate::hooks::HookResult::Continue);
		};
		$(let $param: $ty = $param; )*
	};
	(WindowResize, $ctx:ident, $( $param:ident : $ty:ty ),* $(,)?) => {
		let $crate::hooks::HookEventData::WindowResize { $($param,)* .. } = &$ctx.data else {
			return $crate::hooks::HookAction::Done($crate::hooks::HookResult::Continue);
		};
		$(let $param: $ty = $param; )*
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __async_hook_extract {
	(EditorStart, $owned:ident $(,)?) => {
		let $crate::hooks::OwnedHookContext::EditorStart = $owned else {
			return $crate::hooks::HookResult::Continue;
		};
	};
	(EditorQuit, $owned:ident $(,)?) => {
		let $crate::hooks::OwnedHookContext::EditorQuit = $owned else {
			return $crate::hooks::HookResult::Continue;
		};
	};
	(EditorTick, $owned:ident $(,)?) => {
		let $crate::hooks::OwnedHookContext::EditorTick = $owned else {
			return $crate::hooks::HookResult::Continue;
		};
	};
	(FocusGained, $owned:ident $(,)?) => {
		let $crate::hooks::OwnedHookContext::FocusGained = $owned else {
			return $crate::hooks::HookResult::Continue;
		};
	};
	(FocusLost, $owned:ident $(,)?) => {
		let $crate::hooks::OwnedHookContext::FocusLost = $owned else {
			return $crate::hooks::HookResult::Continue;
		};
	};
	(BufferOpen, $owned:ident, $( $param:ident : $ty:ty ),* $(,)?) => {
		let $crate::hooks::OwnedHookContext::BufferOpen { $($param,)* .. } = $owned else {
			return $crate::hooks::HookResult::Continue;
		};
		$(let $param: $ty = $crate::__hook_param_expr!($ty, $param); )*
	};
	(BufferWritePre, $owned:ident, $( $param:ident : $ty:ty ),* $(,)?) => {
		let $crate::hooks::OwnedHookContext::BufferWritePre { $($param,)* .. } = $owned else {
			return $crate::hooks::HookResult::Continue;
		};
		$(let $param: $ty = $crate::__hook_param_expr!($ty, $param); )*
	};
	(BufferWrite, $owned:ident, $( $param:ident : $ty:ty ),* $(,)?) => {
		let $crate::hooks::OwnedHookContext::BufferWrite { $($param,)* .. } = $owned else {
			return $crate::hooks::HookResult::Continue;
		};
		$(let $param: $ty = $crate::__hook_param_expr!($ty, $param); )*
	};
	(BufferClose, $owned:ident, $( $param:ident : $ty:ty ),* $(,)?) => {
		let $crate::hooks::OwnedHookContext::BufferClose { $($param,)* .. } = $owned else {
			return $crate::hooks::HookResult::Continue;
		};
		$(let $param: $ty = $crate::__hook_param_expr!($ty, $param); )*
	};
	(BufferChange, $owned:ident, $( $param:ident : $ty:ty ),* $(,)?) => {
		let $crate::hooks::OwnedHookContext::BufferChange { $($param,)* .. } = $owned else {
			return $crate::hooks::HookResult::Continue;
		};
		$(let $param: $ty = $crate::__hook_param_expr!($ty, $param); )*
	};
	(ModeChange, $owned:ident, $( $param:ident : $ty:ty ),* $(,)?) => {
		let $crate::hooks::OwnedHookContext::ModeChange { $($param,)* .. } = $owned else {
			return $crate::hooks::HookResult::Continue;
		};
		$(let $param: $ty = $crate::__hook_param_expr!($ty, $param); )*
	};
	(CursorMove, $owned:ident, $( $param:ident : $ty:ty ),* $(,)?) => {
		let $crate::hooks::OwnedHookContext::CursorMove { $($param,)* .. } = $owned else {
			return $crate::hooks::HookResult::Continue;
		};
		$(let $param: $ty = $crate::__hook_param_expr!($ty, $param); )*
	};
	(SelectionChange, $owned:ident, $( $param:ident : $ty:ty ),* $(,)?) => {
		let $crate::hooks::OwnedHookContext::SelectionChange { $($param,)* .. } = $owned else {
			return $crate::hooks::HookResult::Continue;
		};
		$(let $param: $ty = $crate::__hook_param_expr!($ty, $param); )*
	};
	(WindowResize, $owned:ident, $( $param:ident : $ty:ty ),* $(,)?) => {
		let $crate::hooks::OwnedHookContext::WindowResize { $($param,)* .. } = $owned else {
			return $crate::hooks::HookResult::Continue;
		};
		$(let $param: $ty = $crate::__hook_param_expr!($ty, $param); )*
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
/// Actions are the primary unit of editor functionality. Each action has a name,
/// description, and handler that returns an [`ActionResult`]. Actions can optionally
/// have keybindings (via `bindings:`) and result handlers (via `result:`).
///
/// # Forms
///
/// **Basic action** - registers handler only, no keybindings:
/// ```ignore
/// action!(name, { description: "..." }, |ctx| { ... });
/// action!(name, { description: "..." }, handler: my_handler_fn);
/// ```
///
/// **Action with keybindings** - parses KDL binding syntax via [`parse_keybindings!`]:
/// ```ignore
/// action!(name, {
///     description: "...",
///     bindings: r#"normal "x" "ctrl-x""#,
/// }, |ctx| { ... });
/// ```
///
/// **Buffer-ops action** - for actions that delegate to [`BufferOps`] trait methods.
/// Generates the action, keybindings, AND result handler in one declaration:
/// ```ignore
/// action!(split_horizontal, {
///     description: "Split horizontally",
///     bindings: r#"window "s""#,
///     result: SplitHorizontal,
/// }, |ops| ops.split_horizontal());
/// ```
///
/// The `result:` form delegates to [`buffer_ops_handler!`] to generate a
/// [`ResultHandler`] that matches on the specified [`ActionResult`] variant
/// and calls the body with the [`BufferOps`] context.
///
/// [`ActionResult`]: crate::actions::ActionResult
/// [`BufferOps`]: crate::editor_ctx::BufferOps
/// [`ResultHandler`]: crate::editor_ctx::ResultHandler
/// [`parse_keybindings!`]: evildoer_macro::parse_keybindings
/// [`buffer_ops_handler!`]: evildoer_macro::buffer_ops_handler
#[macro_export]
macro_rules! action {
	// Buffer-ops form: action + keybindings + result handler colocated.
	// Delegates handler generation to buffer_ops_handler! proc macro for
	// CamelCase → SCREAMING_SNAKE_CASE slice name derivation.
	($name:ident, {
		description: $desc:expr,
		bindings: $kdl:literal,
		result: $result:ident
		$(,)?
	}, |$ops:ident| $body:expr) => {
		$crate::action!($name, {
			description: $desc,
			bindings: $kdl
		}, |_ctx| $crate::actions::ActionResult::$result);

		evildoer_macro::buffer_ops_handler!($name, $result, $ops, $body);
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
/// bind!(scroll_down, r#"normal "z j""#);
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
///         tracing::info!(path = %path.display(), "Opened buffer");
///     }
/// });
/// ```
#[macro_export]
macro_rules! hook {
	($name:ident, $event:ident, $priority:expr, $desc:expr, mutable |$ctx:ident| $body:expr) => {
		paste::paste! {
			#[allow(clippy::unused_unit)]
			fn [<hook_handler_ $name>](
				$ctx: &mut $crate::hooks::MutableHookContext,
			) -> $crate::hooks::HookAction {
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
				mutability: $crate::hooks::HookMutability::Mutable,
				handler: $crate::hooks::HookHandler::Mutable([<hook_handler_ $name>]),
				source: $crate::RegistrySource::Crate(env!("CARGO_PKG_NAME")),
			};
		}
	};
	($name:ident, $event:ident, $priority:expr, $desc:expr, |$($param:ident : $ty:ty),*| $body:expr) => {
		$crate::hook!($name, $event, $priority, $desc, |ctx| {
			$crate::__hook_extract!($event, ctx, $($param : $ty),*);
			$body
		});
	};
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
				mutability: $crate::hooks::HookMutability::Immutable,
				handler: $crate::hooks::HookHandler::Immutable([<hook_handler_ $name>]),
				source: $crate::RegistrySource::Crate(env!("CARGO_PKG_NAME")),
			};
		}
	};
}

/// Defines an async hook that owns extracted parameters.
#[macro_export]
macro_rules! async_hook {
	($name:ident, $event:ident, $priority:expr, $desc:expr, setup |$ctx:ident| { $($setup:tt)* } async || $body:expr) => {
		$crate::hook!($name, $event, $priority, $desc, |$ctx| {
			$($setup)*
			let owned = $ctx.to_owned();
			$crate::hooks::HookAction::Async(::std::boxed::Box::pin(async move {
				$crate::__async_hook_extract!($event, owned);
				let result = { $body };
				::core::convert::Into::into(result)
			}))
		});
	};
	($name:ident, $event:ident, $priority:expr, $desc:expr, async || $body:expr) => {
		$crate::hook!($name, $event, $priority, $desc, |ctx| {
			let owned = ctx.to_owned();
			$crate::hooks::HookAction::Async(::std::boxed::Box::pin(async move {
				$crate::__async_hook_extract!($event, owned);
				let result = { $body };
				::core::convert::Into::into(result)
			}))
		});
	};
	($name:ident, $event:ident, $priority:expr, $desc:expr, setup |$ctx:ident| { $($setup:tt)* } async |$($param:ident : $ty:ty),*| $body:expr) => {
		$crate::hook!($name, $event, $priority, $desc, |$ctx| {
			$($setup)*
			let owned = $ctx.to_owned();
			$crate::hooks::HookAction::Async(::std::boxed::Box::pin(async move {
				$crate::__async_hook_extract!($event, owned, $($param : $ty),*);
				let result = { $body };
				::core::convert::Into::into(result)
			}))
		});
	};
	($name:ident, $event:ident, $priority:expr, $desc:expr, async |$($param:ident : $ty:ty),*| $body:expr) => {
		$crate::hook!($name, $event, $priority, $desc, |ctx| {
			let owned = ctx.to_owned();
			$crate::hooks::HookAction::Async(::std::boxed::Box::pin(async move {
				$crate::__async_hook_extract!($event, owned, $($param : $ty),*);
				let result = { $body };
				::core::convert::Into::into(result)
			}))
		});
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

/// Registers a symmetric text object where inner == around.
#[macro_export]
macro_rules! symmetric_text_object {
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
	}, $handler:expr) => {
		$crate::text_object!($name, {
			trigger: $trigger,
			$(alt_triggers: $alt_triggers,)?
			$(aliases: $aliases,)?
			description: $desc
			$(, priority: $priority)?
			$(, caps: $caps)?
			$(, flags: $flags)?
			$(, source: $source)?
		}, {
			inner: $handler,
			around: $handler,
		});
	};
}

/// Registers a bracket-pair text object with surround selection.
#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! bracket_pair_object {
	($name:ident, $open:expr, $close:expr, $trigger:expr, $alt_triggers:expr) => {
		paste::paste! {
			fn [<$name _inner>](text: ropey::RopeSlice, pos: usize) -> Option<$crate::Range> {
				crate::movement::select_surround_object(
					text,
					$crate::Range::point(pos),
					$open,
					$close,
					true,
				)
			}

			fn [<$name _around>](text: ropey::RopeSlice, pos: usize) -> Option<$crate::Range> {
				crate::movement::select_surround_object(
					text,
					$crate::Range::point(pos),
					$open,
					$close,
					false,
				)
			}

			$crate::text_object!($name, {
				trigger: $trigger,
				alt_triggers: $alt_triggers,
				description: concat!("Select ", stringify!($name), " block"),
			}, {
				inner: [<$name _inner>],
				around: [<$name _around>],
			});
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
	__opt, __opt_slice, action, bind, command, hook, motion, option, panel, result_handler,
	statusline_segment, text_object,
};
