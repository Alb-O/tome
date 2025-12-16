//! Plugin loader - loads WebAssembly plugins using Extism.

use std::path::{Path, PathBuf};

use extism::{Manifest, PluginBuilder, UserData, Wasm};
use extism::convert::Json;

use super::host::{create_host_functions, PluginHostContext};
use super::registry::{LoadedPlugin, PluginRegistration};
use crate::selection::Selection;

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
    /// Timeout in milliseconds.
    #[serde(default)]
    pub timeout_ms: Option<u64>,
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
                    } else {
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
        }

        plugins
    }

    /// Load a plugin from a .wasm file or manifest.
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

        // Create the shared host context (UserData wraps in Arc<Mutex<>> internally)
        let host_ctx = UserData::new(PluginHostContext::new(
            String::new(),
            &Selection::point(0),
            0,
        ));

        // Build extism manifest
        let wasi_enabled = manifest.as_ref().map(|m| m.wasi).unwrap_or(false);
        let mut extism_manifest = Manifest::new([Wasm::file(&wasm_path)]);
        
        if let Some(ref m) = manifest {
            if !m.allowed_hosts.is_empty() {
                extism_manifest = extism_manifest.with_allowed_hosts(m.allowed_hosts.iter().cloned());
            }
            if let Some(timeout) = m.timeout_ms {
                extism_manifest = extism_manifest.with_timeout(std::time::Duration::from_millis(timeout));
            }
        }

        // Create host functions
        let host_functions = create_host_functions(host_ctx.clone());

        // Build the plugin
        let mut plugin = PluginBuilder::new(extism_manifest)
            .with_wasi(wasi_enabled)
            .with_functions(host_functions)
            .build()
            .map_err(|e| PluginLoadError::InvalidWasm(e.to_string()))?;

        // Call plugin_init to get registration
        let Json(registration): Json<PluginRegistration> = plugin
            .call("plugin_init", &[] as &[u8])
            .map_err(|e| PluginLoadError::InitFailed(e.to_string()))?;

        Ok(LoadedPlugin::new(
            id,
            registration.name.clone(),
            registration.version.clone(),
            wasm_path,
            registration,
            plugin,
            host_ctx,
        ))
    }

    /// Load a plugin directly from wasm bytes (for testing or embedded plugins).
    pub fn load_from_bytes(
        &self,
        id: String,
        wasm_bytes: &[u8],
        wasi: bool,
    ) -> Result<LoadedPlugin, PluginLoadError> {
        let host_ctx = UserData::new(PluginHostContext::new(
            String::new(),
            &Selection::point(0),
            0,
        ));

        let manifest = Manifest::new([Wasm::data(wasm_bytes)]);
        let host_functions = create_host_functions(host_ctx.clone());

        let mut plugin = PluginBuilder::new(manifest)
            .with_wasi(wasi)
            .with_functions(host_functions)
            .build()
            .map_err(|e| PluginLoadError::InvalidWasm(e.to_string()))?;

        let Json(registration): Json<PluginRegistration> = plugin
            .call("plugin_init", &[] as &[u8])
            .map_err(|e| PluginLoadError::InitFailed(e.to_string()))?;

        Ok(LoadedPlugin::new(
            id,
            registration.name.clone(),
            registration.version.clone(),
            PathBuf::from("<memory>"),
            registration,
            plugin,
            host_ctx,
        ))
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
