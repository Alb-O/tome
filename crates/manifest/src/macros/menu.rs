//! Menu registration macros.

/// Registers a menu group in the [`MENU_GROUPS`](crate::menu::MENU_GROUPS) slice.
#[macro_export]
macro_rules! menu_group {
	($name:ident, {
		label: $label:expr
		$(, priority: $priority:expr)?
		$(,)?
	}) => {
		paste::paste! {
			#[allow(non_upper_case_globals)]
			#[linkme::distributed_slice($crate::menu::MENU_GROUPS)]
			static [<MENU_GROUP_ $name>]: $crate::menu::MenuGroupDef = $crate::menu::MenuGroupDef {
				id: concat!(env!("CARGO_PKG_NAME"), "::", stringify!($name)),
				name: stringify!($name),
				label: $label,
				priority: $crate::__opt!($({$priority})?, 50),
				source: $crate::RegistrySource::Crate(env!("CARGO_PKG_NAME")),
			};
		}
	};
}

/// Registers a menu item in the [`MENU_ITEMS`](crate::menu::MENU_ITEMS) slice.
#[macro_export]
macro_rules! menu_item {
	($name:ident, {
		group: $group:expr,
		label: $label:expr,
		command: $command:expr
		$(, shortcut: $shortcut:expr)?
		$(, priority: $priority:expr)?
		$(,)?
	}) => {
		paste::paste! {
			#[allow(non_upper_case_globals)]
			#[linkme::distributed_slice($crate::menu::MENU_ITEMS)]
			static [<MENU_ITEM_ $name>]: $crate::menu::MenuItemDef = $crate::menu::MenuItemDef {
				id: concat!(env!("CARGO_PKG_NAME"), "::", stringify!($name)),
				name: stringify!($name),
				group: $group,
				label: $label,
				command: $command,
				shortcut: $crate::__opt!($({Some($shortcut)})?, None),
				priority: $crate::__opt!($({$priority})?, 50),
				source: $crate::RegistrySource::Crate(env!("CARGO_PKG_NAME")),
			};
		}
	};
}
