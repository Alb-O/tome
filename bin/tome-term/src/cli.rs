use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::styles::cli_styles;

#[derive(Parser, Debug)]
#[command(name = "tome")]
#[command(about = "A modal text editor")]
#[command(version)]
#[command(styles = cli_styles())]
pub struct Cli {
	/// File to edit (opens scratch buffer if omitted)
	pub file: Option<PathBuf>,

	/// Execute an Ex command at startup (e.g. "acp.start")
	#[arg(long = "ex")]
	pub ex: Option<String>,

	/// Exit immediately after running `--ex`
	#[arg(long)]
	pub quit_after_ex: bool,

	#[command(subcommand)]
	pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
	/// Plugin management
	Plugin(PluginArgs),
}

#[derive(Args, Debug)]
pub struct PluginArgs {
	#[command(subcommand)]
	pub command: PluginCommands,
}

#[derive(Subcommand, Debug)]
pub enum PluginCommands {
	/// Register a plugin in development mode
	DevAdd {
		/// Path to the plugin crate
		#[arg(long)]
		path: PathBuf,
	},
	/// Add a plugin from a local path
	Add {
		#[arg(long)]
		from_path: PathBuf,
	},
	/// Remove a plugin
	Remove { id: String },
	/// Enable a plugin
	Enable { id: String },
	/// Disable a plugin
	Disable { id: String },
	/// Reload a plugin (experimental)
	Reload { id: String },
}
