//! FFI-safe components for Kakoune.
//!
//! Provides C ABI exports for hash, UTF-8, and Unicode functions
//! that are binary-compatible with Kakoune's C++ implementations.

mod hash;
mod unicode;
mod utf8;

pub use hash::*;
pub use unicode::*;
pub use utf8::*;
