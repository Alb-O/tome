use tome_macro::tome_api;

#[cfg(feature = "host")]
use crate::ext::plugins::{PluginHostContext, PendingOp};

#[cfg(feature = "host")]
#[tome_api(PluginHostContext)]
/// API for file operations.
pub trait FileApi {
    /// Open a file in the editor.
    fn editor_open_file(&mut self, path: String);
}

#[cfg(not(feature = "host"))]
#[tome_api]
/// API for file operations.
pub trait FileApi {
    /// Open a file in the editor.
    fn editor_open_file(&mut self, path: String);
}


#[cfg(feature = "host")]
impl FileApi for PluginHostContext {
    fn editor_open_file(&mut self, path: String) {
        self.pending_ops.push(PendingOp::OpenFile(path));
    }
}

#[cfg(feature = "host")]
use crate::ext::plugins::registry::{HOST_FUNCTION_FACTORIES, HostFunctionFactory};
#[cfg(feature = "host")]
use linkme::distributed_slice;

#[cfg(feature = "host")]
#[distributed_slice(HOST_FUNCTION_FACTORIES)]
static FILE_API_REGISTRATION: HostFunctionFactory = create_fileapi_host_functions;

