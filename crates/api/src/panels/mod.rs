//! Runtime panel management.
//!
//! This module provides the [`PanelRegistry`] which manages panel instances at runtime.
//! Panel types are registered at compile time via [`panel!`](evildoer_manifest::panel),
//! and this registry handles creating, storing, and accessing panel instances.

mod registry;

use std::any::Any;

pub use registry::PanelRegistry;

/// Factory function type for creating panel instances.
pub type PanelFactory = fn() -> Box<dyn Any + Send>;

/// Registration for a panel factory.
///
/// Links a panel type name to its factory function.
pub struct PanelFactoryDef {
	/// Panel type name (must match a [`PanelDef`](evildoer_manifest::PanelDef) name).
	pub name: &'static str,
	/// Factory function to create new instances.
	pub factory: PanelFactory,
}

/// Distributed slice for panel factories.
#[linkme::distributed_slice]
pub static PANEL_FACTORIES: [PanelFactoryDef];

/// Finds a panel factory by name.
pub fn find_factory(name: &str) -> Option<&'static PanelFactoryDef> {
	PANEL_FACTORIES.iter().find(|f| f.name == name)
}
