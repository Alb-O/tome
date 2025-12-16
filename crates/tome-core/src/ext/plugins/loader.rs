//! Plugin loader - loads WebAssembly plugins using Extism.

use std::path::{Path, PathBuf};

use super::registry::LoadedPlugin;

/// Plugin manifest for loading from a directory.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PluginManifest {
    /// Plugin ID (must be unique).
    pub id: String,
    /// Path to the .wasm file (relative to manifest).
    pub wasm: String,
    /// Whether to enable WASI.
    #[serde(default)]
    pub wasi: bool,
    /// Allowed hosts for HTTP requests.
    #[serde(default)]
    pub allowed_hosts: Vec<String>,
}

/// Loads plugins from WebAssembly files.
pub struct PluginLoader {
    /// Directory containing plugins.
    plugin_dir: PathBuf,
}

impl PluginLoader {
    pub fn new(plugin_dir: impl Into<PathBuf>) -> Self {
        Self {
            plugin_dir: plugin_dir.into(),
        }
    }

    /// Discover all plugins in the plugin directory.
    pub fn discover(&self) -> Vec<PathBuf> {
        let mut plugins = Vec::new();

        if let Ok(entries) = std::fs::read_dir(&self.plugin_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|e| e == "wasm") {
                    plugins.push(path);
                } else if path.is_dir() {
                    // Check for manifest.json in subdirectory
                    let manifest_path = path.join("manifest.json");
                    if manifest_path.exists() {
                        plugins.push(manifest_path);
                    }
                    // Or a single .wasm file
                    if let Ok(sub_entries) = std::fs::read_dir(&path) {
                        for sub_entry in sub_entries.flatten() {
                            let sub_path = sub_entry.path();
                            if sub_path.extension().is_some_and(|e| e == "wasm") {
                                plugins.push(sub_path);
                                break;
                            }
                        }
                    }
                }
            }
        }

        plugins
    }

    /// Load a plugin from a .wasm file or manifest.
    ///
    /// This is a stub that will be implemented when extism is added as a dependency.
    pub fn load(&self, path: &Path) -> Result<LoadedPlugin, PluginLoadError> {
        let (wasm_path, manifest) = if path.extension().is_some_and(|e| e == "json") {
            // Load manifest
            let content = std::fs::read_to_string(path)
                .map_err(|e| PluginLoadError::Io(e.to_string()))?;
            let manifest: PluginManifest = serde_json::from_str(&content)
                .map_err(|e| PluginLoadError::InvalidManifest(e.to_string()))?;
            let wasm_path = path.parent().unwrap_or(path).join(&manifest.wasm);
            (wasm_path, Some(manifest))
        } else {
            (path.to_path_buf(), None)
        };

        if !wasm_path.exists() {
            return Err(PluginLoadError::NotFound(wasm_path.display().to_string()));
        }

        // Read wasm bytes (for validation, actual loading needs extism)
        let _wasm_bytes = std::fs::read(&wasm_path)
            .map_err(|e| PluginLoadError::Io(e.to_string()))?;

        // Generate plugin ID from filename if no manifest
        let id = manifest
            .as_ref()
            .map(|m| m.id.clone())
            .unwrap_or_else(|| {
                wasm_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("unknown")
                    .to_string()
            });

        // TODO: When extism is integrated:
        // 1. Create Plugin from wasm_bytes with host functions
        // 2. Call plugin_init export to get registration
        // 3. Parse registration JSON into PluginRegistration
        // 4. Create LoadedPlugin with the extism::Plugin stored

        // For now, return a stub
        Ok(LoadedPlugin {
            id: id.clone(),
            name: id,
            version: "0.0.0".to_string(),
            path: wasm_path,
            actions: Vec::new(),
            commands: Vec::new(),
            hooks: Vec::new(),
            keybindings: Vec::new(),
        })
    }
}

#[derive(Debug, Clone)]
pub enum PluginLoadError {
    NotFound(String),
    Io(String),
    InvalidManifest(String),
    InvalidWasm(String),
    InitFailed(String),
}

impl std::fmt::Display for PluginLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginLoadError::NotFound(p) => write!(f, "plugin not found: {}", p),
            PluginLoadError::Io(e) => write!(f, "I/O error: {}", e),
            PluginLoadError::InvalidManifest(e) => write!(f, "invalid manifest: {}", e),
            PluginLoadError::InvalidWasm(e) => write!(f, "invalid wasm: {}", e),
            PluginLoadError::InitFailed(e) => write!(f, "plugin init failed: {}", e),
        }
    }
}

impl std::error::Error for PluginLoadError {}
