//! Text object registration macros.
//!
//! [`text_object!`], [`symmetric_text_object!`], and [`bracket_pair_object!`].

/// Registers a text object in the [`TEXT_OBJECTS`](crate::text_objects::TEXT_OBJECTS) slice.
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
			#[linkme::distributed_slice($crate::text_objects::TEXT_OBJECTS)]
			static [<OBJ_ $name>]: $crate::text_objects::TextObjectDef = $crate::text_objects::TextObjectDef::new(
				concat!(env!("CARGO_PKG_NAME"), "::", stringify!($name)),
				stringify!($name),
				$crate::__opt_slice!($({$aliases})?),
				$desc,
				$crate::__opt!($({$priority})?, 0),
				$crate::__opt!($({$source})?, $crate::RegistrySource::Crate(env!("CARGO_PKG_NAME"))),
				$crate::__opt_slice!($({$caps})?),
				$crate::__opt!($({$flags})?, $crate::flags::NONE),
				$trigger,
				$crate::__opt_slice!($({$alt_triggers})?),
				$inner,
				$around,
			);
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
#[allow(
	clippy::crate_in_macro_def,
	reason = "macro is internal and always called from this crate"
)]
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
