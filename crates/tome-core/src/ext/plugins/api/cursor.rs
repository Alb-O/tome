use tome_macro::tome_api;

#[cfg(feature = "host")]
use crate::ext::plugins::{PluginHostContext, PendingOp};

#[cfg(feature = "host")]
#[tome_api(PluginHostContext)]
/// API for manipulating the cursor and selection.
pub trait CursorApi {
    /// Get the primary cursor position (byte offset).
    fn editor_get_cursor(&self) -> u64;
    /// Set the primary cursor position (byte offset).
    fn editor_set_cursor(&mut self, pos: u64);
    /// Get the current selection as a JSON string.
    fn editor_get_selection(&self) -> String;
    /// Set the current selection from a JSON string.
    fn editor_set_selection(&mut self, json: String);
}

#[cfg(not(feature = "host"))]
#[tome_api]
/// API for manipulating the cursor and selection.
pub trait CursorApi {
    /// Get the primary cursor position (byte offset).
    fn editor_get_cursor(&self) -> u64;
    /// Set the primary cursor position (byte offset).
    fn editor_set_cursor(&mut self, pos: u64);
    /// Get the current selection as a JSON string.
    fn editor_get_selection(&self) -> String;
    /// Set the current selection from a JSON string.
    fn editor_set_selection(&mut self, json: String);
}


#[cfg(feature = "host")]
impl CursorApi for PluginHostContext {
    fn editor_get_cursor(&self) -> u64 {
        self.cursor as u64
    }

    fn editor_set_cursor(&mut self, pos: u64) {
        self.pending_ops.push(PendingOp::SetCursor(pos as usize));
    }

    fn editor_get_selection(&self) -> String {
        let json = serde_json::json!({
            "anchor": self.selection_anchor,
            "head": self.selection_head
        });
        json.to_string()
    }

    fn editor_set_selection(&mut self, json: String) {
        #[derive(serde::Deserialize)]
        struct SelectionJson {
            anchor: usize,
            head: usize,
        }
        
        if let Ok(sel) = serde_json::from_str::<SelectionJson>(&json) {
            self.pending_ops.push(PendingOp::SetSelection {
                anchor: sel.anchor,
                head: sel.head,
            });
        }
    }
}

#[cfg(feature = "host")]
use crate::ext::plugins::registry::{HOST_FUNCTION_FACTORIES, HostFunctionFactory};
#[cfg(feature = "host")]
use linkme::distributed_slice;

#[cfg(feature = "host")]
#[distributed_slice(HOST_FUNCTION_FACTORIES)]
static CURSOR_API_REGISTRATION: HostFunctionFactory = create_cursorapi_host_functions;

