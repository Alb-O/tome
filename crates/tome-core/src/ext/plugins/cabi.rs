//! Minimal C-ABI plugin surface for Tome.
//!
//! This is intentionally tiny: a single entry point returning a guest vtable
//! and consuming a host vtable. The goal is “suckless” runtime plugins without
//! WASM. ABI is `extern "C"` + POD types only.

use std::ffi::{c_char, CStr};
use std::path::Path;

use libloading::Library;

/// ABI version for compatibility checks.
pub const TOME_C_ABI_VERSION: u32 = 1;

/// Status codes returned across the ABI boundary.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TomeStatus {
    Ok = 0,
    Failed = 1,
    Incompatible = 2,
}

/// Host function table passed to the plugin.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TomeHostV1 {
    /// ABI version of the host.
    pub abi_version: u32,
    /// Optional logging hook from guest -> host.
    pub log: Option<extern "C" fn(*const c_char)>,
}

/// Guest function table returned by the plugin.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TomeGuestV1 {
    /// ABI version the guest expects.
    pub abi_version: u32,
    /// Optional initialization hook. Called once after load.
    pub init: Option<extern "C" fn() -> TomeStatus>,
}

/// Signature of the plugin entry point.
pub type TomePluginEntry = unsafe extern "C" fn(
    host: *const TomeHostV1,
    out_guest: *mut TomeGuestV1,
) -> TomeStatus;

/// Errors while loading a C-ABI plugin.
#[derive(Debug)]
pub enum CAbiLoadError {
    Load(libloading::Error),
    MissingEntry,
    Incompatible { host: u32, guest: u32 },
    InitFailed,
}

impl std::fmt::Display for CAbiLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CAbiLoadError::Load(e) => write!(f, "dlopen failed: {e}"),
            CAbiLoadError::MissingEntry => write!(f, "missing entry symbol 'tome_plugin_entry'"),
            CAbiLoadError::Incompatible { host, guest } => {
                write!(f, "incompatible abi version: host={host} guest={guest}")
            }
            CAbiLoadError::InitFailed => write!(f, "plugin init failed"),
        }
    }
}

impl std::error::Error for CAbiLoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CAbiLoadError::Load(e) => Some(e),
            _ => None,
        }
    }
}

/// Loaded C-ABI plugin handle.
pub struct CAbiPlugin {
    _lib: Library,
    guest: TomeGuestV1,
}

impl CAbiPlugin {
    /// Call the guest `init` if present.
    pub fn init(&self) -> Result<(), CAbiLoadError> {
        if let Some(init) = self.guest.init {
            let status = init();
            if status != TomeStatus::Ok {
                return Err(CAbiLoadError::InitFailed);
            }
        }
        Ok(())
    }
}

/// Load a C-ABI plugin from a shared library file.
pub fn load_c_abi_plugin(path: &Path) -> Result<CAbiPlugin, CAbiLoadError> {
    let lib = unsafe { Library::new(path) }.map_err(CAbiLoadError::Load)?;

    let entry: libloading::Symbol<TomePluginEntry> = unsafe {
        lib.get(b"tome_plugin_entry\0").map_err(|_| CAbiLoadError::MissingEntry)?
    };

    let host = TomeHostV1 { abi_version: TOME_C_ABI_VERSION, log: Some(host_log) };
    let mut guest = TomeGuestV1::default();

    let status = unsafe { entry(&host, &mut guest) };
    if status != TomeStatus::Ok {
        return Err(CAbiLoadError::InitFailed);
    }

    if guest.abi_version != TOME_C_ABI_VERSION {
        return Err(CAbiLoadError::Incompatible { host: TOME_C_ABI_VERSION, guest: guest.abi_version });
    }

    let plugin = CAbiPlugin { _lib: lib, guest };
    plugin.init()?;
    Ok(plugin)
}

extern "C" fn host_log(ptr: *const c_char) {
    if ptr.is_null() {
        return;
    }
    if let Ok(msg) = unsafe { CStr::from_ptr(ptr) }.to_str() {
        eprintln!("[tome plugin] {msg}");
    }
}
