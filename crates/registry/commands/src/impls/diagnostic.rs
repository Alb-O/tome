//! Diagnostic navigation commands.

use futures::future::LocalBoxFuture;
use xeno_registry_notifications::keys;

use crate::{CommandContext, CommandError, CommandOutcome, command};

command!(
	diagnostics,
	{ aliases: &["diag"], description: "Toggle diagnostics panel" },
	handler: cmd_diagnostics
);

/// Handler for the `:diagnostics` command.
fn cmd_diagnostics<'a>(
	ctx: &'a mut CommandContext<'a>,
) -> LocalBoxFuture<'a, Result<CommandOutcome, CommandError>> {
	Box::pin(async move {
		ctx.editor.toggle_diagnostics_panel();
		Ok(CommandOutcome::Ok)
	})
}

command!(
	diagnostic_next,
	{ aliases: &["dn"], description: "Go to next diagnostic" },
	handler: cmd_diagnostic_next
);

/// Handler for the `:diagnostic-next` command.
fn cmd_diagnostic_next<'a>(
	ctx: &'a mut CommandContext<'a>,
) -> LocalBoxFuture<'a, Result<CommandOutcome, CommandError>> {
	Box::pin(async move {
		match ctx.editor.goto_next_diagnostic() {
			Some(message) => {
				ctx.emit(keys::diagnostic_message::call(&message));
			}
			None => {
				ctx.emit(keys::no_diagnostics);
			}
		}
		Ok(CommandOutcome::Ok)
	})
}

command!(
	diagnostic_prev,
	{ aliases: &["dp"], description: "Go to previous diagnostic" },
	handler: cmd_diagnostic_prev
);

/// Handler for the `:diagnostic-prev` command.
fn cmd_diagnostic_prev<'a>(
	ctx: &'a mut CommandContext<'a>,
) -> LocalBoxFuture<'a, Result<CommandOutcome, CommandError>> {
	Box::pin(async move {
		match ctx.editor.goto_prev_diagnostic() {
			Some(message) => {
				ctx.emit(keys::diagnostic_message::call(&message));
			}
			None => {
				ctx.emit(keys::no_diagnostics);
			}
		}
		Ok(CommandOutcome::Ok)
	})
}
