use tome_macro::tome_api;

#[cfg(feature = "host")]
use crate::ext::plugins::PluginHostContext;

#[cfg(feature = "host")]
#[tome_api(PluginHostContext)]
/// API for system interactions.
pub trait SystemApi {
    /// Display a message to the user.
    fn editor_message(&mut self, msg: String);
}

#[cfg(not(feature = "host"))]
#[tome_api]
/// API for system interactions.
pub trait SystemApi {
    /// Display a message to the user.
    fn editor_message(&mut self, msg: String);
}


#[cfg(feature = "host")]
impl SystemApi for PluginHostContext {
    fn editor_message(&mut self, msg: String) {
        self.messages.push(msg);
    }
}

#[cfg(feature = "host")]
use crate::ext::plugins::registry::{HOST_FUNCTION_FACTORIES, HostFunctionFactory};
#[cfg(feature = "host")]
use linkme::distributed_slice;

#[cfg(feature = "host")]
#[distributed_slice(HOST_FUNCTION_FACTORIES)]
static SYSTEM_API_REGISTRATION: HostFunctionFactory = create_systemapi_host_functions;

