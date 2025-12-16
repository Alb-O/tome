use tome_macro::tome_api;

#[cfg(feature = "host")]
use crate::ext::plugins::PluginHostContext;
#[cfg(feature = "host")]
use crate::movement;
#[cfg(feature = "host")]
use ropey::Rope;

#[cfg(feature = "host")]
#[tome_api(PluginHostContext)]
/// API for searching text in the buffer.
pub trait SearchApi {
    /// Search for a regex pattern. Returns the (start, end) byte offsets of the match.
    fn editor_search(&self, pattern: String, reverse: bool) -> Option<(usize, usize)>;
}

#[cfg(not(feature = "host"))]
#[tome_api]
/// API for searching text in the buffer.
pub trait SearchApi {
    /// Search for a regex pattern. Returns the (start, end) byte offsets of the match.
    fn editor_search(&self, pattern: String, reverse: bool) -> Option<(usize, usize)>;
}


#[cfg(feature = "host")]
impl SearchApi for PluginHostContext {
    fn editor_search(&self, pattern: String, reverse: bool) -> Option<(usize, usize)> {
        let rope = Rope::from_str(&self.text);
        let slice = rope.slice(..);
        
        let result = if reverse {
            movement::find_prev(slice, &pattern, self.cursor)
        } else {
            // Forward search usually starts after cursor?
            // movement::find_next takes "at" position.
            // If we are at 0, we search from 0?
            // The editor implementation usually does cursor + 1 for next.
            movement::find_next(slice, &pattern, self.cursor)
        };
        
        match result {
            Ok(Some(range)) => Some((range.anchor, range.head)),
            _ => None
        }
    }
}

#[cfg(feature = "host")]
use crate::ext::plugins::registry::{HOST_FUNCTION_FACTORIES, HostFunctionFactory};
#[cfg(feature = "host")]
use linkme::distributed_slice;

#[cfg(feature = "host")]
#[distributed_slice(HOST_FUNCTION_FACTORIES)]
static SEARCH_API_REGISTRATION: HostFunctionFactory = create_searchapi_host_functions;

