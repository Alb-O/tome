use tome_macro::tome_api;

#[cfg(feature = "host")]
use crate::ext::plugins::{PluginHostContext, PendingOp};

#[cfg(feature = "host")]
#[tome_api(PluginHostContext)]
/// API for accessing and modifying the editor buffer.
pub trait BufferApi {
    /// Get the entire text content of the buffer.
    fn editor_get_text(&self) -> String;
    /// Insert text at the current cursor position.
    fn editor_insert(&mut self, text: String);
    /// Delete the currently selected text.
    fn editor_delete(&mut self);
}

#[cfg(not(feature = "host"))]
#[tome_api]
/// API for accessing and modifying the editor buffer.
pub trait BufferApi {
    /// Get the entire text content of the buffer.
    fn editor_get_text(&self) -> String;
    /// Insert text at the current cursor position.
    fn editor_insert(&mut self, text: String);
    /// Delete the currently selected text.
    fn editor_delete(&mut self);
}


#[cfg(feature = "host")]
impl BufferApi for PluginHostContext {
    fn editor_get_text(&self) -> String {
        self.text.clone()
    }

    fn editor_insert(&mut self, text: String) {
        self.pending_ops.push(PendingOp::Insert(text));
    }

    fn editor_delete(&mut self) {
        self.pending_ops.push(PendingOp::Delete);
    }
}

#[cfg(feature = "host")]
use crate::ext::plugins::registry::{HOST_FUNCTION_FACTORIES, HostFunctionFactory};
#[cfg(feature = "host")]
use linkme::distributed_slice;

#[cfg(feature = "host")]
#[distributed_slice(HOST_FUNCTION_FACTORIES)]
static BUFFER_API_REGISTRATION: HostFunctionFactory = create_bufferapi_host_functions;

