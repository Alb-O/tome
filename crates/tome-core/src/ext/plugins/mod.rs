//! External plugin system using Extism WebAssembly runtime.
//!
//! This module allows users to write extensions in any language that compiles
//! to WebAssembly (Rust, Go, JavaScript, Python, etc.) and have them interact
//! with Tome's internal systems.
//!
//! # Architecture
//!
//! Plugins are WebAssembly modules that can:
//! - Register actions, commands, hooks, and keybindings
//! - Call host functions to interact with the editor
//! - Be loaded from `.wasm` files or manifests
//!
//! # Host Functions
//!
//! Plugins can call these host functions (exposed by Tome):
//!
//! | Function | Description |
//! |----------|-------------|
//! | `editor_get_text` | Get document text |
//! | `editor_get_selection` | Get current selection |
//! | `editor_set_selection` | Set selection |
//! | `editor_insert` | Insert text at cursor |
//! | `editor_delete` | Delete selection |
//! | `editor_message` | Show message to user |
//! | `editor_get_cursor` | Get cursor position |
//! | `editor_set_cursor` | Set cursor position |
//! | `config_get` | Get configuration value |
//! | `config_set` | Set configuration value |
//!
//! # Plugin Exports
//!
//! Plugins must export these functions:
//!
//! | Function | Description |
//! |----------|-------------|
//! | `plugin_init` | Called when plugin loads, returns registration JSON |
//! | `on_action` | Called when a registered action is triggered |
//! | `on_hook` | Called when a registered hook fires |
//! | `on_command` | Called when a registered command is executed |

mod host;
mod loader;
mod registry;

pub use host::PluginHostContext;
pub use loader::{PluginLoader, PluginManifest};
pub use registry::{PluginRegistry, LoadedPlugin};
