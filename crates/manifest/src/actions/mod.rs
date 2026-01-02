//! Action definitions and execution context.
//!
//! Action types are defined in the registry actions crate.

pub use evildoer_registry::actions::*;

impl crate::RegistryMetadata for evildoer_registry::actions::ActionDef {
	fn id(&self) -> &'static str {
		self.id
	}

	fn name(&self) -> &'static str {
		self.name
	}

	fn priority(&self) -> i16 {
		self.priority
	}

	fn source(&self) -> crate::RegistrySource {
		match self.source {
			evildoer_registry::RegistrySource::Builtin => crate::RegistrySource::Builtin,
			evildoer_registry::RegistrySource::Crate(name) => crate::RegistrySource::Crate(name),
			evildoer_registry::RegistrySource::Runtime => crate::RegistrySource::Runtime,
		}
	}
}
