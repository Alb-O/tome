use futures::future::LocalBoxFuture;

use crate::{Capability, CommandError, CommandOutcome, EditorOps, RegistrySource};

pub struct CommandDef {
	pub id: &'static str,
	pub name: &'static str,
	pub aliases: &'static [&'static str],
	pub description: &'static str,
	pub handler: for<'a> fn(
		_: &'a mut CommandContext<'a>,
	) -> LocalBoxFuture<'a, Result<CommandOutcome, CommandError>>,
	pub user_data: Option<&'static (dyn std::any::Any + Sync)>,
	pub priority: i16,
	pub source: RegistrySource,
	pub required_caps: &'static [Capability],
	pub flags: u32,
}

pub struct CommandContext<'a> {
	pub editor: &'a mut dyn EditorOps,
	pub args: &'a [&'a str],
	pub count: usize,
	pub register: Option<char>,
	pub user_data: Option<&'static (dyn std::any::Any + Sync)>,
}

impl<'a> crate::editor_ctx::MessageAccess for CommandContext<'a> {
	fn notify(&mut self, type_id: &str, msg: &str) {
		self.editor.notify(type_id, msg);
	}

	fn clear_message(&mut self) {
		self.editor.clear_message();
	}
}

impl<'a> CommandContext<'a> {
	pub fn require_user_data<T: std::any::Any + Sync>(&self) -> Result<&'static T, CommandError> {
		self.user_data
			.and_then(|d| {
				let any: &dyn std::any::Any = d;
				any.downcast_ref::<T>()
			})
			.ok_or_else(|| {
				CommandError::Other(format!(
					"Missing or invalid user data for command (expected {})",
					std::any::type_name::<T>()
				))
			})
	}
}

/// Command flags for optional behavior hints.
pub mod flags {
	/// No special flags.
	pub const NONE: u32 = 0;
}

crate::impl_registry_metadata!(CommandDef);
