use proc_macro::TokenStream;

mod api;
mod dispatch;
mod keybindings;
mod notification;

/// Generates WASM host/guest bridge code for extension APIs.
///
/// When applied to a trait, generates:
/// - Guest-side methods that call into the host via WASM imports
/// - Host-side functions that dispatch to the trait implementation
///
/// ```ignore
/// #[evildoer_api(ExtensionHostContext)]
/// pub trait ExtensionApi {
///     fn notify(&mut self, message: &str);
///     fn get_buffer_content(&self) -> String;
/// }
/// ```
#[proc_macro_attribute]
pub fn evildoer_api(attr: TokenStream, item: TokenStream) -> TokenStream {
	api::evildoer_api(attr, item)
}

/// Registers a notification type with the notification system.
///
/// ```ignore
/// register_notification!(INFO_NOTIFICATION, "info",
///     level: evildoer_manifest::notifications::Level::Info,
///     semantic: evildoer_manifest::SEMANTIC_INFO
/// );
/// ```
#[proc_macro]
pub fn register_notification(input: TokenStream) -> TokenStream {
	notification::register_notification(input)
}

/// Derives dispatch infrastructure for `ActionResult`.
///
/// Generates handler slice declarations (`RESULT_*_HANDLERS`), a `dispatch_result`
/// function, and `is_terminal_safe` method.
///
/// Attributes:
/// - `#[handler(Foo)]` - Use `RESULT_FOO_HANDLERS` instead of deriving from variant name
/// - `#[terminal_safe]` - Mark variant as safe to execute when terminal is focused
///
/// ```ignore
/// #[derive(DispatchResult)]
/// pub enum ActionResult {
///     #[terminal_safe]
///     Ok,
///     #[handler(Quit)]
///     Quit,
///     Motion(Selection),
/// }
/// ```
#[proc_macro_derive(DispatchResult, attributes(handler, terminal_safe))]
pub fn derive_dispatch_result(input: TokenStream) -> TokenStream {
	dispatch::derive_dispatch_result(input)
}

/// Parses KDL keybinding definitions at compile time.
///
/// ```kdl
/// normal "h" "left" "ctrl-h"
/// insert "left"
/// goto "h"
/// ```
///
/// Called internally by `action!` macro:
///
/// ```ignore
/// action!(
///     move_left,
///     { description: "Move cursor left", bindings: r#"normal "h" "left""# },
///     |ctx| { ... }
/// );
/// ```
#[proc_macro]
pub fn parse_keybindings(input: TokenStream) -> TokenStream {
	keybindings::parse_keybindings(input)
}
