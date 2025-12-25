use futures::future::LocalBoxFuture;
use tome_manifest::{CommandContext, CommandError, CommandOutcome};

use crate::command;

command!(edit, { aliases: &["e"], description: "Edit a file" }, handler: cmd_edit);

fn cmd_edit<'a>(
	ctx: &'a mut CommandContext<'a>,
) -> LocalBoxFuture<'a, Result<CommandOutcome, CommandError>> {
	Box::pin(async move {
		if ctx.args.is_empty() {
			return Err(CommandError::MissingArgument("filename"));
		}
		ctx.warning(&format!("edit {} - not yet implemented", ctx.args[0]));
		Ok(CommandOutcome::Ok)
	})
}
