use extism_pdk::*;
use serde::{Deserialize, Serialize};
use tome_core::ext::plugins::{
    api::{buffer, system, search, file, config}, // Import the API modules
    ActionInput, ActionOutput, CommandInput, HookInput,
};

#[derive(Serialize, Deserialize)]
pub struct PluginRegistration {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub actions: Vec<ActionRegistration>,
    #[serde(default)]
    pub commands: Vec<CommandRegistration>,
    #[serde(default)]
    pub hooks: Vec<String>,
    #[serde(default)]
    pub keybindings: Vec<PluginKeybinding>,
}

#[derive(Serialize, Deserialize)]
pub struct ActionRegistration {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommandRegistration {
    pub name: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct PluginKeybinding {
    pub mode: String,
    pub key: String,
    pub action: String,
}

#[plugin_fn]
pub fn plugin_init() -> FnResult<Json<PluginRegistration>> {
    Ok(Json(PluginRegistration {
        name: "Demo Plugin".to_string(),
        version: "0.1.0".to_string(),
        actions: vec![
            ActionRegistration {
                name: "demo_insert_hello".to_string(),
                description: "Insert 'Hello from Plugin!'".to_string(),
            },
            ActionRegistration {
                name: "demo_upper_selection".to_string(),
                description: "Uppercase selection".to_string(),
            },
        ],
        commands: vec![
            CommandRegistration {
                name: "hello".to_string(),
                aliases: vec![],
                description: "Say hello via command".to_string(),
            },
            CommandRegistration {
                name: "search_test".to_string(),
                aliases: vec![],
                description: "Test search api".to_string(),
            },
            CommandRegistration {
                name: "config_test".to_string(),
                aliases: vec![],
                description: "Test config api".to_string(),
            },
            CommandRegistration {
                name: "file_test".to_string(),
                aliases: vec![],
                description: "Test file open api".to_string(),
            },
        ],
        hooks: vec![],
        keybindings: vec![],
    }))
}

#[plugin_fn]
pub fn on_action(Json(input): Json<ActionInput>) -> FnResult<Json<ActionOutput>> {
    match input.action_name.as_str() {
        "demo_insert_hello" => {
            // Demonstrate calling host function directly
            system::host::editor_message("Executing demo_insert_hello from plugin...".to_string());
            
            Ok(Json(ActionOutput {
                insert_text: Some("Hello from Plugin!".to_string()),
                message: Some("Executed demo action".to_string()),
                ..Default::default()
            }))
        }
        "demo_upper_selection" => {
            // Use host function to get text instead of input (just to test it)
            let text = buffer::host::editor_get_text();
            
            let anchor = input.editor.selection_anchor;
            let head = input.editor.selection_head;
            
            let (from, to) = if anchor < head { (anchor, head) } else { (head, anchor) };
            
            if from < to && to <= text.len() {
                let selected = &text[from..to];
                let upper = selected.to_uppercase();
                
                Ok(Json(ActionOutput {
                    insert_text: Some(upper),
                    message: Some("Uppercased selection".to_string()),
                    ..Default::default()
                }))
            } else {
                Ok(Json(ActionOutput {
                    message: Some("No selection".to_string()),
                    ..Default::default()
                }))
            }
        }
        _ => Ok(Json(ActionOutput::default())),
    }
}

#[plugin_fn]
pub fn on_command(Json(input): Json<CommandInput>) -> FnResult<Json<ActionOutput>> {
    match input.command_name.as_str() {
        "hello" => {
             Ok(Json(ActionOutput {
                insert_text: Some("Hello from Command!".to_string()),
                message: Some("Ran hello command".to_string()),
                ..Default::default()
            }))
        }
        "search_test" => {
            let pattern = if input.args.is_empty() { "test".to_string() } else { input.args[0].clone() };
            if let Some((anchor, head)) = search::host::editor_search(pattern.clone(), false) {
                Ok(Json(ActionOutput {
                    set_selection: Some((anchor, head)),
                    set_cursor: Some(head),
                    message: Some(format!("Found '{}' at {}-{}", pattern, anchor, head)),
                    ..Default::default()
                }))
            } else {
                Ok(Json(ActionOutput {
                    message: Some(format!("'{}' not found", pattern)),
                    ..Default::default()
                }))
            }
        }
        "config_test" => {
            let key = if input.args.is_empty() { "text_width".to_string() } else { input.args[0].clone() };
            if let Some(val) = config::host::editor_get_config(key.clone()) {
                 Ok(Json(ActionOutput {
                    message: Some(format!("Config '{}': {}", key, val)),
                    ..Default::default()
                }))
            } else {
                 Ok(Json(ActionOutput {
                    message: Some(format!("Config '{}' not set", key)),
                    ..Default::default()
                }))
            }
        }
        "file_test" => {
            if input.args.is_empty() {
                 return Ok(Json(ActionOutput {
                    message: Some("Usage: file_test <path>".to_string()),
                    ..Default::default()
                }));
            }
            let path = input.args[0].clone();
            file::host::editor_open_file(path.clone());
            Ok(Json(ActionOutput {
                message: Some(format!("Requesting open: {}", path)),
                ..Default::default()
            }))
        }
        _ => Ok(Json(ActionOutput::default())),
    }
}

#[plugin_fn]
pub fn on_hook(Json(_input): Json<HookInput>) -> FnResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({})))
}
