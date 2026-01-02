//! Menu bar registry: groups, items, and registration macros.

mod def;
mod impls;
#[doc(hidden)]
mod macros;

pub use def::*;

/// Where a registry item was defined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RegistrySource {
	Builtin,
	Crate(&'static str),
	Runtime,
}

impl core::fmt::Display for RegistrySource {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Builtin => write!(f, "builtin"),
			Self::Crate(name) => write!(f, "crate:{name}"),
			Self::Runtime => write!(f, "runtime"),
		}
	}
}
