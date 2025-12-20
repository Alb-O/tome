use crate::command;
use crate::ext::index::{diagnostics, get_registry};
use crate::ext::{CommandContext, CommandError, CommandOutcome};

command!(ext_diag, { aliases: &["ext.diag"], description: "Show extension system diagnostics" }, handler: cmd_ext_diag);
command!(ext_doctor, { aliases: &["ext.doctor"], description: "Check for extension collisions and suggest fixes" }, handler: cmd_ext_doctor);

fn cmd_ext_diag(ctx: &mut CommandContext) -> Result<CommandOutcome, CommandError> {
	let reg = get_registry();
	let mut out = Vec::new();

	out.push("--- Extension Registry ---".to_string());
	out.push(format!("Commands: {}", reg.commands.by_name.len()));
	out.push(format!("Actions:  {}", reg.actions.by_name.len()));
	out.push(format!("Motions:  {}", reg.motions.by_name.len()));
	out.push(format!("Objects:  {}", reg.text_objects.by_name.len()));

	let diag = diagnostics();
	if !diag.collisions.is_empty() {
		out.push(format!("\nTotal Collisions: {}", diag.collisions.len()));
		for c in &diag.collisions {
			out.push(format!(
				"  {} collision on '{}': {} shadowed by {} (priority {} vs {})",
				c.source_type,
				c.key,
				c.shadowed_id,
				c.winner_id,
				c.shadowed_priority,
				c.winner_priority
			));
		}
	} else {
		out.push("\nNo collisions detected.".to_string());
	}

	ctx.message(&out.join("\n"));
	Ok(CommandOutcome::Ok)
}

fn cmd_ext_doctor(ctx: &mut CommandContext) -> Result<CommandOutcome, CommandError> {
	let diag = diagnostics();
	if diag.collisions.is_empty() {
		ctx.message("All good! No collisions found.");
		return Ok(CommandOutcome::Ok);
	}

	let mut out = Vec::new();
	out.push(format!("Found {} collisions:", diag.collisions.len()));

	for c in &diag.collisions {
		out.push(format!("\nCollision on {} '{}':", c.source_type, c.key));
		out.push(format!(
			"  - Winner:  {} (priority {})",
			c.winner_id, c.winner_priority
		));
		out.push(format!(
			"  - Loser:   {} (priority {})",
			c.shadowed_id, c.shadowed_priority
		));

		if c.winner_priority == c.shadowed_priority {
			out.push(
				"  Suggestion: Increase priority of the one you want to win, or rename one."
					.to_string(),
			);
		} else {
			out.push(format!(
				"  Note: {} wins due to higher priority.",
				c.winner_id
			));
		}
	}

	ctx.message(&out.join("\n"));
	Ok(CommandOutcome::Ok)
}
