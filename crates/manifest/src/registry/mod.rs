//! Common registry infrastructure for compile-time registration.
//!
//! This module provides the shared foundation for all registry types (actions,
//! commands, motions, text objects, hooks, options, etc.).
//!
//! # Architecture
//!
//! Registries use [`linkme`] distributed slices for zero-cost compile-time
//! registration. Each registry type defines:
//!
//! 1. A `*Def` struct with registry-specific fields plus common metadata
//! 2. A public macro (e.g., `action!`, `motion!`) for ergonomic registration
//! 3. A distributed slice to collect all definitions
//!
//! The `impl_registry_metadata!` macro reduces boilerplate for implementing
//! the `RegistryMetadata` trait on new definition types.

// Internal macro definitions are exported via #[macro_export] in register.rs
mod register;
