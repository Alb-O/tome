//! Motion primitive definitions.
//!
//! Motions are the fundamental cursor movement operations (char, word, line, etc.).
//! They're composed by actions to implement editor commands.

use linkme::distributed_slice;

use crate::{Capability, Range, RegistrySource};

/// Handler signature for motion primitives.
///
/// Parameters:
/// - `text`: The document text as a rope slice
/// - `range`: Current cursor range (anchor..head)
/// - `count`: Repeat count (1 if not specified)
/// - `extend`: Whether to extend selection (vs move cursor)
///
/// Returns the new range after applying the motion.
pub type MotionHandler = fn(ropey::RopeSlice, Range, usize, bool) -> Range;

/// Definition of a motion primitive.
///
/// Motions are registered via the `motion!` macro and looked up by name
/// from action handlers.
pub struct MotionDef {
	pub id: &'static str,
	pub name: &'static str,
	pub aliases: &'static [&'static str],
	pub description: &'static str,
	pub handler: MotionHandler,
	pub priority: i16,
	pub source: RegistrySource,
	pub required_caps: &'static [Capability],
	pub flags: u32,
}

impl MotionDef {
	/// Creates a new motion definition.
	#[doc(hidden)]
	#[allow(
		clippy::too_many_arguments,
		reason = "builder pattern is not const-compatible"
	)]
	pub const fn new(
		id: &'static str,
		name: &'static str,
		aliases: &'static [&'static str],
		description: &'static str,
		priority: i16,
		source: RegistrySource,
		required_caps: &'static [Capability],
		flags: u32,
		handler: MotionHandler,
	) -> Self {
		Self {
			id,
			name,
			aliases,
			description,
			handler,
			priority,
			source,
			required_caps,
			flags,
		}
	}
}

crate::impl_registry_metadata!(MotionDef);

/// Registry of all motion definitions.
#[distributed_slice]
pub static MOTIONS: [MotionDef];

/// Find a motion by name.
pub fn find(name: &str) -> Option<&'static MotionDef> {
	MOTIONS
		.iter()
		.find(|m| m.name == name || m.aliases.contains(&name))
}

/// Get all registered motions.
pub fn all() -> &'static [MotionDef] {
	&MOTIONS
}
