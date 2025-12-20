use tome_core::command;
use tome_core::ext::{CommandContext, CommandError, CommandOutcome};

command!(plugins, { aliases: &["plugin"], description: "Manage plugins" }, handler: cmd_plugins);

fn cmd_plugins(ctx: &mut CommandContext) -> Result<CommandOutcome, CommandError> {
	ctx.editor
		.plugin_command(ctx.args)
		.map(|_| CommandOutcome::Ok)
}
