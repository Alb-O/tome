//! Registry-first organization for Evildoer editor extensions.
//!
//! This crate re-exports all registry sub-crates for convenient access.
//! Each registry is a separate crate under `crates/registry/`:
//!
//! - [`menus`] - Menu bar groups and items
//! - [`motions`] - Cursor movement primitives

pub use {evildoer_registry_menus as menus, evildoer_registry_motions as motions};
