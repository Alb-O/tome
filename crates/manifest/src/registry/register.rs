//! Internal registration infrastructure.
//!
//! Provides the `impl_registry_metadata!` macro for implementing the
//! `RegistryMetadata` trait on registry definition types.

/// Implements `RegistryMetadata` for a Def type.
///
/// Reduces boilerplate when defining new registry types. The type must have
/// `id`, `name`, `priority`, and `source` fields.
///
/// # Example
///
/// ```ignore
/// pub struct MyDef {
///     pub id: &'static str,
///     pub name: &'static str,
///     pub priority: i16,
///     pub source: RegistrySource,
///     // ... other fields
/// }
///
/// impl_registry_metadata!(MyDef);
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! impl_registry_metadata {
	($type:ty) => {
		impl $crate::RegistryMetadata for $type {
			fn id(&self) -> &'static str {
				self.id
			}
			fn name(&self) -> &'static str {
				self.name
			}
			fn priority(&self) -> i16 {
				self.priority
			}
			fn source(&self) -> $crate::RegistrySource {
				self.source
			}
		}
	};
}
