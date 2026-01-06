//! Inlay hints options.

use linkme::distributed_slice;

use crate::{
	OPTIONS, OptionDef, OptionScope, OptionType, OptionValue, RegistrySource, TypedOptionKey,
};

/// Typed handle for the `INLAY_HINTS_ENABLED` option.
///
/// Whether to display inlay hints (type annotations, parameter names, etc.).
pub const INLAY_HINTS_ENABLED: TypedOptionKey<bool> =
	TypedOptionKey::new(&__OPT_INLAY_HINTS_ENABLED);

#[allow(non_upper_case_globals)]
#[distributed_slice(OPTIONS)]
static __OPT_INLAY_HINTS_ENABLED: OptionDef = OptionDef {
	id: concat!(env!("CARGO_PKG_NAME"), "::INLAY_HINTS_ENABLED"),
	name: "INLAY_HINTS_ENABLED",
	kdl_key: "inlay-hints",
	description: "Enable inlay hints (type annotations, parameter names).",
	value_type: OptionType::Bool,
	default: || OptionValue::Bool(true),
	scope: OptionScope::Buffer,
	priority: 0,
	source: RegistrySource::Crate(env!("CARGO_PKG_NAME")),
	validator: None,
};
