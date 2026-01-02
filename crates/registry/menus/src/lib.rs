//! Menu bar registry: groups, items, and registration macros.

mod def;
mod impls;
#[doc(hidden)]
mod macros;

pub use def::*;
pub use evildoer_registry_motions::{RegistryMetadata, RegistrySource, impl_registry_metadata};
