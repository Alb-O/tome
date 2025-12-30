//! Runtime panel management.
//!
//! This module provides the [`PanelRegistry`] which manages panel instances at runtime.
//! Panel types are registered at compile time via [`panel!`](evildoer_manifest::panel),
//! and this registry handles creating, storing, and accessing panel instances.
//!
//! Panel factories are registered in the [`PANEL_FACTORIES`](evildoer_manifest::PANEL_FACTORIES)
//! distributed slice, either inline via `panel!` macro's `factory:` parameter, or
//! separately for types defined in downstream crates.

mod registry;

use evildoer_manifest::{PANEL_FACTORIES, PanelFactoryDef};
use linkme::distributed_slice;
pub use registry::PanelRegistry;

use crate::debug::DebugPanel;
use crate::terminal::TerminalBuffer;

// Register factories for panel types defined in this crate.
// These use the PANEL_FACTORIES slice from evildoer-manifest.

#[distributed_slice(PANEL_FACTORIES)]
static TERMINAL_FACTORY: PanelFactoryDef = PanelFactoryDef {
	name: "terminal",
	factory: || Box::new(TerminalBuffer::new()),
};

#[distributed_slice(PANEL_FACTORIES)]
static DEBUG_FACTORY: PanelFactoryDef = PanelFactoryDef {
	name: "debug",
	factory: || Box::new(DebugPanel::new()),
};
