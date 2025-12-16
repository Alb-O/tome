//! C-ABI plugin integration.
//! Runtime plugins are native `cdylib` libraries that implement `tome_plugin_entry`.

#[cfg(feature = "host")]
pub mod cabi;

#[cfg(feature = "host")]
pub use cabi::{load_c_abi_plugin, CAbiLoadError, CAbiPlugin};
#[cfg(feature = "host")]
pub use tome_cabi_types::{TomeGuestV1, TomeHostV1, TomePluginEntry, TomeStatus, TOME_C_ABI_VERSION};
