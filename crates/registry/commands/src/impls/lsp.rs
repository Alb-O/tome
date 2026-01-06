//! LSP-related commands.

use futures::future::LocalBoxFuture;
use xeno_registry_notifications::keys;

use crate::{CommandContext, CommandError, CommandOutcome, command};

command!(
	hover,
	{ aliases: &[], description: "Show hover information at cursor" },
	handler: cmd_hover
);

/// Handler for the `:hover` command.
fn cmd_hover<'a>(
	ctx: &'a mut CommandContext<'a>,
) -> LocalBoxFuture<'a, Result<CommandOutcome, CommandError>> {
	Box::pin(async move {
		let shown = ctx.editor.show_hover().await;
		if !shown {
			ctx.emit(keys::no_hover_info);
		}
		Ok(CommandOutcome::Ok)
	})
}

command!(
	complete,
	{ aliases: &["completion"], description: "Trigger completion at cursor" },
	handler: cmd_complete
);

/// Handler for the `:complete` command.
fn cmd_complete<'a>(
	ctx: &'a mut CommandContext<'a>,
) -> LocalBoxFuture<'a, Result<CommandOutcome, CommandError>> {
	Box::pin(async move {
		let shown = ctx.editor.trigger_completion().await;
		if !shown {
			// No notification needed for completion - UI feedback is implicit
		}
		Ok(CommandOutcome::Ok)
	})
}

command!(
	definition,
	{ aliases: &["def", "gd"], description: "Go to definition at cursor" },
	handler: cmd_definition
);

/// Handler for the `:definition` command.
fn cmd_definition<'a>(
	ctx: &'a mut CommandContext<'a>,
) -> LocalBoxFuture<'a, Result<CommandOutcome, CommandError>> {
	Box::pin(async move {
		let jumped = ctx.editor.goto_definition().await;
		if !jumped {
			ctx.emit(keys::no_definition);
		}
		Ok(CommandOutcome::Ok)
	})
}

command!(
	references,
	{ aliases: &["ref"], description: "Find references at cursor" },
	handler: cmd_references
);

/// Handler for the `:references` command.
fn cmd_references<'a>(
	ctx: &'a mut CommandContext<'a>,
) -> LocalBoxFuture<'a, Result<CommandOutcome, CommandError>> {
	Box::pin(async move {
		let found = ctx.editor.find_references().await;
		if !found {
			ctx.emit(keys::no_references);
		}
		Ok(CommandOutcome::Ok)
	})
}

command!(
	code_actions,
	{ aliases: &["ca", "actions"], description: "Show code actions at cursor" },
	handler: cmd_code_actions
);

/// Handler for the `:code_actions` command.
fn cmd_code_actions<'a>(
	ctx: &'a mut CommandContext<'a>,
) -> LocalBoxFuture<'a, Result<CommandOutcome, CommandError>> {
	Box::pin(async move {
		let shown = ctx.editor.show_code_actions().await;
		if !shown {
			ctx.emit(keys::no_code_actions);
		}
		Ok(CommandOutcome::Ok)
	})
}
