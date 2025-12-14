//! FFI-safe hash functions for Kakoune.
//!
//! Provides C ABI exports for FNV-1a and MurmurHash3 implementations
//! that are binary-compatible with Kakoune's C++ hash functions.

mod hash;

pub use hash::*;
