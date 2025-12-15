//! Built-in command registrations for command-line mode.
//!
//! These are the `:commands` that users can invoke from the command prompt.
//! The internal `Command` enum in keymap.rs handles key-bound actions,
//! while these `CommandDef` entries handle user-facing command-line commands.

use linkme::distributed_slice;

use super::{CommandContext, CommandDef, CommandResult, COMMANDS};

// Note: Most commands need access to Editor state that isn't in CommandContext.
// For now, we register placeholders. The actual implementation will be in tome-term.
// CommandContext provides: text, selection, args, count, register
// It does NOT provide: file I/O, undo/redo, message display

#[distributed_slice(COMMANDS)]
static CMD_HELP: CommandDef = CommandDef {
    name: "help",
    aliases: &["h"],
    description: "Show help for commands",
    handler: cmd_help,
};

fn cmd_help(_ctx: &mut CommandContext) -> CommandResult {
    Ok(())
}

#[distributed_slice(COMMANDS)]
static CMD_QUIT: CommandDef = CommandDef {
    name: "quit",
    aliases: &["q"],
    description: "Quit the editor",
    handler: cmd_quit,
};

fn cmd_quit(_ctx: &mut CommandContext) -> CommandResult {
    // This is handled specially by the command executor
    // Returning Ok signals the intent to quit
    Ok(())
}

#[distributed_slice(COMMANDS)]
static CMD_QUIT_FORCE: CommandDef = CommandDef {
    name: "quit!",
    aliases: &["q!"],
    description: "Quit without saving",
    handler: cmd_quit_force,
};

fn cmd_quit_force(_ctx: &mut CommandContext) -> CommandResult {
    Ok(())
}

#[distributed_slice(COMMANDS)]
static CMD_WRITE: CommandDef = CommandDef {
    name: "write",
    aliases: &["w"],
    description: "Write buffer to file",
    handler: cmd_write,
};

fn cmd_write(_ctx: &mut CommandContext) -> CommandResult {
    // File I/O is handled by the terminal layer
    Ok(())
}

#[distributed_slice(COMMANDS)]
static CMD_WRITE_QUIT: CommandDef = CommandDef {
    name: "wq",
    aliases: &["x"],
    description: "Write and quit",
    handler: cmd_write_quit,
};

fn cmd_write_quit(_ctx: &mut CommandContext) -> CommandResult {
    Ok(())
}

#[distributed_slice(COMMANDS)]
static CMD_EDIT: CommandDef = CommandDef {
    name: "edit",
    aliases: &["e"],
    description: "Edit a file",
    handler: cmd_edit,
};

fn cmd_edit(_ctx: &mut CommandContext) -> CommandResult {
    // Requires filename argument from ctx.args
    Ok(())
}

#[distributed_slice(COMMANDS)]
static CMD_BUFFER: CommandDef = CommandDef {
    name: "buffer",
    aliases: &["b"],
    description: "Switch to buffer",
    handler: cmd_buffer,
};

fn cmd_buffer(_ctx: &mut CommandContext) -> CommandResult {
    Ok(())
}

#[distributed_slice(COMMANDS)]
static CMD_BUFFER_NEXT: CommandDef = CommandDef {
    name: "buffer-next",
    aliases: &["bn"],
    description: "Go to next buffer",
    handler: cmd_buffer_next,
};

fn cmd_buffer_next(_ctx: &mut CommandContext) -> CommandResult {
    Ok(())
}

#[distributed_slice(COMMANDS)]
static CMD_BUFFER_PREV: CommandDef = CommandDef {
    name: "buffer-previous",
    aliases: &["bp"],
    description: "Go to previous buffer",
    handler: cmd_buffer_prev,
};

fn cmd_buffer_prev(_ctx: &mut CommandContext) -> CommandResult {
    Ok(())
}

#[distributed_slice(COMMANDS)]
static CMD_DELETE_BUFFER: CommandDef = CommandDef {
    name: "delete-buffer",
    aliases: &["db"],
    description: "Delete current buffer",
    handler: cmd_delete_buffer,
};

fn cmd_delete_buffer(_ctx: &mut CommandContext) -> CommandResult {
    Ok(())
}
