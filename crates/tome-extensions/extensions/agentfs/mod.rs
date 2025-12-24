//! AgentFS extension for Tome.
//!
//! Provides commands to connect to and disconnect from AgentFS databases.

mod commands;

use std::sync::Arc;

use agentfs_sdk::{AgentFS, AgentFSOptions, FileSystem, HostFS};
use linkme::distributed_slice;

use tome_api::editor::extensions::{EXTENSIONS, ExtensionInitDef};

pub struct AgentFsManager {
	pub current_agent_id: Option<String>,
}

impl AgentFsManager {
	pub fn new() -> Self {
		Self {
			current_agent_id: None,
		}
	}

	pub async fn connect(&mut self, id_or_path: &str) -> anyhow::Result<Arc<dyn FileSystem>> {
		let options = AgentFSOptions::resolve(id_or_path)?;
		let agent = AgentFS::open(options).await?;
		self.current_agent_id = Some(id_or_path.to_string());
		Ok(Arc::new(agent.fs))
	}

	pub fn disconnect(&mut self) -> anyhow::Result<Arc<dyn FileSystem>> {
		self.current_agent_id = None;
		Ok(Arc::new(HostFS::new(std::env::current_dir()?)?))
	}
}

#[distributed_slice(EXTENSIONS)]
static AGENTFS_INIT: ExtensionInitDef = ExtensionInitDef {
	id: "agentfs",
	priority: 100,
	init: |map| {
		map.insert(AgentFsManager::new());
	},
};
