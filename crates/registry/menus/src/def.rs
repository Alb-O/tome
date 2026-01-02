use linkme::distributed_slice;

use crate::{RegistrySource, impl_registry_metadata};

/// A top-level menu group (e.g., "File", "Edit").
pub struct MenuGroupDef {
	pub id: &'static str,
	pub name: &'static str,
	pub label: &'static str,
	pub priority: i16,
	pub source: RegistrySource,
}

/// A menu item within a group.
pub struct MenuItemDef {
	pub id: &'static str,
	pub name: &'static str,
	pub group: &'static str,
	pub label: &'static str,
	pub command: &'static str,
	pub shortcut: Option<&'static str>,
	pub priority: i16,
	pub source: RegistrySource,
}

impl_registry_metadata!(MenuGroupDef);
impl_registry_metadata!(MenuItemDef);

#[distributed_slice]
pub static MENU_GROUPS: [MenuGroupDef];

#[distributed_slice]
pub static MENU_ITEMS: [MenuItemDef];

/// Returns all registered menu groups.
pub fn all_groups() -> &'static [MenuGroupDef] {
	&MENU_GROUPS
}

/// Returns menu items for a given group name.
pub fn items_for_group(group_name: &str) -> impl Iterator<Item = &'static MenuItemDef> + '_ {
	MENU_ITEMS
		.iter()
		.filter(move |item| item.group == group_name)
}
