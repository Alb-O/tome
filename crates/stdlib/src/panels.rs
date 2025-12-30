//! Panel type definitions.
//!
//! Defines the built-in panel types (terminal, debug) using the [`panel!`] macro.

use evildoer_manifest::panel;

panel!(terminal, {
	description: "Embedded terminal emulator",
	mode_name: "TERMINAL",
	layer: 1,
	sticky: true,
});

panel!(debug, {
	description: "Debug log viewer",
	mode_name: "DEBUG",
	layer: 2,
	sticky: true,
});
