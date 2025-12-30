//! Panel definitions and runtime management.
//!
//! This module defines the built-in panels (terminal, debug) using the [`panel!`] macro
//! with inline factories, and provides [`PanelRegistry`] for runtime instance management.

mod registry;

pub use registry::PanelRegistry;

use evildoer_manifest::panel;

use crate::debug::DebugPanel;
use crate::terminal::TerminalBuffer;

panel!(terminal, {
	description: "Embedded terminal emulator",
	mode_name: "TERMINAL",
	layer: 1,
	sticky: true,
	factory: || Box::new(TerminalBuffer::new()),
});

panel!(debug, {
	description: "Debug log viewer",
	mode_name: "DEBUG",
	layer: 2,
	sticky: true,
	factory: || Box::new(DebugPanel::new()),
});
