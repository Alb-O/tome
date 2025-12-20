use crate::ext::{CommandContext, CommandOutcome, CommandError};

pub fn test_notify(ctx: &mut CommandContext) -> Result<CommandOutcome, CommandError> {
	ctx.editor.notify("warn", "This is a test notification via distributed slices!");
	Ok(CommandOutcome::Ok)
}

#[cfg(feature = "host")]
use crate::ext::CommandDef;
#[cfg(feature = "host")]
#[::linkme::distributed_slice(crate::ext::COMMANDS)]
static TEST_NOTIFY_CMD: CommandDef = CommandDef {
	name: "test-notify",
	aliases: &[],
	description: "Test the new notification system",
	handler: test_notify,
	user_data: None,
};
