use futures::future::LocalBoxFuture;
use tome_api::editor::Editor;
use tome_manifest::{CommandContext, CommandError, CommandOutcome};
use tome_stdlib::{NotifyINFOExt, command};

use crate::agentfs::AgentFsManager;

command!(agent_connect, {
	aliases: &["agent.connect", "agent.c"],
	description: "Connect to an AgentFS database"
}, handler: cmd_agent_connect);

fn cmd_agent_connect<'a>(
	ctx: &'a mut CommandContext<'a>,
) -> LocalBoxFuture<'a, Result<CommandOutcome, CommandError>> {
	Box::pin(async move {
		let id_or_path = ctx
			.args
			.first()
			.ok_or(CommandError::MissingArgument("agent id or path"))?;

		let editor = ctx.require_editor_mut();
		if let Some(manager) = editor.extensions.get_mut::<AgentFsManager>() {
			match manager.connect(id_or_path).await {
				Ok(fs) => {
					editor.set_filesystem(fs);
					ctx.info(&format!("Connected to agent: {}", id_or_path));
					Ok(CommandOutcome::Ok)
				}
				Err(e) => Err(CommandError::Failed(format!("Failed to connect: {}", e))),
			}
		} else {
			Err(CommandError::Failed(
				"AgentFS extension not loaded".to_string(),
			))
		}
	})
}

command!(agent_disconnect, {
	aliases: &["agent.disconnect", "agent.d"],
	description: "Disconnect from AgentFS and revert to HostFS"
}, handler: cmd_agent_disconnect);

fn cmd_agent_disconnect<'a>(
	ctx: &'a mut CommandContext<'a>,
) -> LocalBoxFuture<'a, Result<CommandOutcome, CommandError>> {
	Box::pin(async move {
		let editor = ctx.require_editor_mut();
		if let Some(manager) = editor.extensions.get_mut::<AgentFsManager>() {
			match manager.disconnect() {
				Ok(fs) => {
					editor.set_filesystem(fs);
					ctx.info("Disconnected from AgentFS");
					Ok(CommandOutcome::Ok)
				}
				Err(e) => Err(CommandError::Failed(format!("Failed to disconnect: {}", e))),
			}
		} else {
			Err(CommandError::Failed(
				"AgentFS extension not loaded".to_string(),
			))
		}
	})
}

trait CommandContextExt {
	fn require_editor_mut(&mut self) -> &mut Editor;
}

impl<'a> CommandContextExt for CommandContext<'a> {
	fn require_editor_mut(&mut self) -> &mut Editor {
		// SAFETY: We know that in tome-term, EditorOps is implemented by Editor
		unsafe { &mut *(self.editor as *mut dyn tome_manifest::EditorOps as *mut Editor) }
	}
}
