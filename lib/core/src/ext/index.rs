use std::collections::HashMap;
use std::sync::OnceLock;

use crate::ext::{
	ACTIONS, ActionDef, COMMANDS, CommandDef, MOTIONS, MotionDef, TEXT_OBJECTS, TextObjectDef,
};

pub struct RegistryIndex<T: 'static> {
	pub by_id: HashMap<&'static str, &'static T>,
	pub by_name: HashMap<&'static str, &'static T>,
	pub by_alias: HashMap<&'static str, &'static T>,
	pub by_trigger: HashMap<char, &'static T>,
	pub collisions: Vec<Collision>,
}

#[derive(Debug, Clone)]
pub struct Collision {
	pub key: String,
	pub first_id: &'static str,
	pub second_id: &'static str,
	pub source: &'static str, // "name" or "alias" or "trigger"
}

impl<T: 'static> Default for RegistryIndex<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T: 'static> RegistryIndex<T> {
	pub fn new() -> Self {
		Self {
			by_id: HashMap::new(),
			by_name: HashMap::new(),
			by_alias: HashMap::new(),
			by_trigger: HashMap::new(),
			collisions: Vec::new(),
		}
	}
}

pub struct ExtensionRegistry {
	pub commands: RegistryIndex<CommandDef>,
	pub actions: RegistryIndex<ActionDef>,
	pub motions: RegistryIndex<MotionDef>,
	pub text_objects: RegistryIndex<TextObjectDef>,
}

static REGISTRY: OnceLock<ExtensionRegistry> = OnceLock::new();

pub fn get_registry() -> &'static ExtensionRegistry {
	REGISTRY.get_or_init(build_registry)
}

fn build_registry() -> ExtensionRegistry {
	let mut commands: RegistryIndex<CommandDef> = RegistryIndex::new();
	let mut sorted_commands: Vec<_> = COMMANDS.iter().collect();
	// Deterministic ordering: higher priority first, then ID
	sorted_commands.sort_by(|a, b| b.priority.cmp(&a.priority).then(a.id.cmp(b.id)));

	for cmd in sorted_commands {
		if let Some(existing) = commands.by_id.get(cmd.id) {
			commands.collisions.push(Collision {
				key: cmd.id.to_string(),
				first_id: existing.id,
				second_id: cmd.id,
				source: "id",
			});
		} else {
			commands.by_id.insert(cmd.id, cmd);
		}

		if let Some(existing) = commands.by_name.get(cmd.name) {
			commands.collisions.push(Collision {
				key: cmd.name.to_string(),
				first_id: existing.id,
				second_id: cmd.id,
				source: "name",
			});
		} else {
			commands.by_name.insert(cmd.name, cmd);
		}

		for alias in cmd.aliases {
			if let Some(existing) = commands.by_alias.get(alias) {
				commands.collisions.push(Collision {
					key: alias.to_string(),
					first_id: existing.id,
					second_id: cmd.id,
					source: "alias",
				});
			} else {
				commands.by_alias.insert(alias, cmd);
			}
		}
	}

	let mut actions: RegistryIndex<ActionDef> = RegistryIndex::new();
	let mut sorted_actions: Vec<_> = ACTIONS.iter().collect();
	sorted_actions.sort_by(|a, b| b.priority.cmp(&a.priority).then(a.id.cmp(b.id)));

	for action in sorted_actions {
		if let Some(existing) = actions.by_id.get(action.id) {
			actions.collisions.push(Collision {
				key: action.id.to_string(),
				first_id: existing.id,
				second_id: action.id,
				source: "id",
			});
		} else {
			actions.by_id.insert(action.id, action);
		}

		if let Some(existing) = actions.by_name.get(action.name) {
			actions.collisions.push(Collision {
				key: action.name.to_string(),
				first_id: existing.id,
				second_id: action.id,
				source: "name",
			});
		} else {
			actions.by_name.insert(action.name, action);
		}
	}

	let mut motions: RegistryIndex<MotionDef> = RegistryIndex::new();
	let mut sorted_motions: Vec<_> = MOTIONS.iter().collect();
	sorted_motions.sort_by(|a, b| b.priority.cmp(&a.priority).then(a.id.cmp(b.id)));

	for motion in sorted_motions {
		if let Some(existing) = motions.by_id.get(motion.id) {
			motions.collisions.push(Collision {
				key: motion.id.to_string(),
				first_id: existing.id,
				second_id: motion.id,
				source: "id",
			});
		} else {
			motions.by_id.insert(motion.id, motion);
		}

		if let Some(existing) = motions.by_name.get(motion.name) {
			motions.collisions.push(Collision {
				key: motion.name.to_string(),
				first_id: existing.id,
				second_id: motion.id,
				source: "name",
			});
		} else {
			motions.by_name.insert(motion.name, motion);
		}
	}

	let mut text_objects: RegistryIndex<TextObjectDef> = RegistryIndex::new();
	let mut sorted_objects: Vec<_> = TEXT_OBJECTS.iter().collect();
	sorted_objects.sort_by(|a, b| b.priority.cmp(&a.priority).then(a.id.cmp(b.id)));

	for obj in sorted_objects {
		if let Some(existing) = text_objects.by_id.get(obj.id) {
			text_objects.collisions.push(Collision {
				key: obj.id.to_string(),
				first_id: existing.id,
				second_id: obj.id,
				source: "id",
			});
		} else {
			text_objects.by_id.insert(obj.id, obj);
		}

		if let Some(existing) = text_objects.by_name.get(obj.name) {
			text_objects.collisions.push(Collision {
				key: obj.name.to_string(),
				first_id: existing.id,
				second_id: obj.id,
				source: "name",
			});
		} else {
			text_objects.by_name.insert(obj.name, obj);
		}

		// Index by primary trigger
		if let Some(existing) = text_objects.by_trigger.get(&obj.trigger) {
			text_objects.collisions.push(Collision {
				key: obj.trigger.to_string(),
				first_id: existing.id,
				second_id: obj.id,
				source: "trigger",
			});
		} else {
			text_objects.by_trigger.insert(obj.trigger, obj);
		}

		// Index by alternative triggers
		for trigger in obj.alt_triggers {
			if let Some(existing) = text_objects.by_trigger.get(trigger) {
				text_objects.collisions.push(Collision {
					key: trigger.to_string(),
					first_id: existing.id,
					second_id: obj.id,
					source: "trigger",
				});
			} else {
				text_objects.by_trigger.insert(*trigger, obj);
			}
		}
	}

	let registry = ExtensionRegistry {
		commands,
		actions,
		motions,
		text_objects,
	};

	if cfg!(debug_assertions) {
		let diag = diagnostics_internal(&registry);
		if !diag.collisions.is_empty() {
			let mut msg = String::from("Extension collisions detected in debug build:\n");
			for c in &diag.collisions {
				msg.push_str(&format!(
					"  {} collision on '{}': {} shadowed by {} (priority {} vs {})\n",
					c.source_type,
					c.key,
					c.shadowed_id,
					c.winner_id,
					c.shadowed_priority,
					c.winner_priority
				));
			}
			msg.push_str("Please resolve these collisions by renaming or adjusting priorities.");
			panic!("{}", msg);
		}
	}

	registry
}

pub fn find_command(name: &str) -> Option<&'static CommandDef> {
	let reg = get_registry();
	reg.commands
		.by_name
		.get(name)
		.or_else(|| reg.commands.by_alias.get(name))
		.copied()
}

pub fn find_action(name: &str) -> Option<&'static ActionDef> {
	let reg = get_registry();
	reg.actions
		.by_name
		.get(name)
		.or_else(|| reg.actions.by_alias.get(name))
		.copied()
}

pub fn find_motion(name: &str) -> Option<&'static MotionDef> {
	let reg = get_registry();
	reg.motions.by_name.get(name).copied()
}

pub fn find_text_object_by_name(name: &str) -> Option<&'static TextObjectDef> {
	let reg = get_registry();
	reg.text_objects.by_name.get(name).copied()
}

pub fn find_text_object_by_trigger(trigger: char) -> Option<&'static TextObjectDef> {
	let reg = get_registry();
	reg.text_objects.by_trigger.get(&trigger).copied()
}

pub struct DiagnosticReport {
	pub collisions: Vec<CollisionReport>,
}

pub struct CollisionReport {
	pub key: String,
	pub winner_id: &'static str,
	pub shadowed_id: &'static str,
	pub source_type: &'static str,
	pub winner_priority: i16,
	pub shadowed_priority: i16,
}

fn diagnostics_internal(reg: &ExtensionRegistry) -> DiagnosticReport {
	let mut reports = Vec::new();

	macro_rules! collect {
		($index:expr) => {
			for c in &$index.collisions {
				let winner = $index.by_id.get(c.first_id).unwrap();
				let shadowed = $index.by_id.get(c.second_id).unwrap();
				reports.push(CollisionReport {
					key: c.key.clone(),
					winner_id: c.first_id,
					shadowed_id: c.second_id,
					source_type: c.source,
					winner_priority: winner.priority,
					shadowed_priority: shadowed.priority,
				});
			}
		};
	}

	collect!(reg.commands);
	collect!(reg.actions);
	collect!(reg.motions);
	collect!(reg.text_objects);

	DiagnosticReport {
		collisions: reports,
	}
}

pub fn diagnostics() -> DiagnosticReport {
	diagnostics_internal(get_registry())
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ext::Capability;

	#[test]
	fn test_no_unimplemented_capabilities() {
		let reg = get_registry();
		let unimplemented = [Capability::Jump, Capability::Macro, Capability::Transform];

		for cmd in reg.commands.by_id.values() {
			for cap in cmd.required_caps {
				assert!(
					!unimplemented.contains(cap),
					"Command '{}' requires unimplemented capability: {:?}",
					cmd.id,
					cap
				);
			}
		}

		for action in reg.actions.by_id.values() {
			for cap in action.required_caps {
				assert!(
					!unimplemented.contains(cap),
					"Action '{}' requires unimplemented capability: {:?}",
					action.id,
					cap
				);
			}
		}
	}
}
