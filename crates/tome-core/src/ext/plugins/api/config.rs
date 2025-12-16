use tome_macro::tome_api;

#[cfg(feature = "host")]
use crate::ext::plugins::PluginHostContext;

#[cfg(feature = "host")]
#[tome_api(PluginHostContext)]
pub trait ConfigApi {
    fn editor_get_config(&self, key: String) -> Option<String>;
}

#[cfg(not(feature = "host"))]
#[tome_api]
pub trait ConfigApi {
    fn editor_get_config(&self, key: String) -> Option<String>;
}

#[cfg(feature = "host")]
impl ConfigApi for PluginHostContext {
    fn editor_get_config(&self, key: String) -> Option<String> {
        self.config.get(&key).cloned()
    }
}

#[cfg(feature = "host")]
use crate::ext::plugins::registry::{HOST_FUNCTION_FACTORIES, HostFunctionFactory};
#[cfg(feature = "host")]
use linkme::distributed_slice;

#[cfg(feature = "host")]
#[distributed_slice(HOST_FUNCTION_FACTORIES)]
static CONFIG_API_REGISTRATION: HostFunctionFactory = create_configapi_host_functions;

