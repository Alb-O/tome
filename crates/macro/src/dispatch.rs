//! ActionResult dispatch derive macro.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, parse_macro_input};

pub fn derive_dispatch_result(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as DeriveInput);
	let enum_name = &input.ident;

	let Data::Enum(data) = &input.data else {
		return syn::Error::new_spanned(&input, "DispatchResult can only be derived for enums")
			.to_compile_error()
			.into();
	};

	let mut slice_names: Vec<syn::Ident> = Vec::new();
	let mut match_arms = Vec::new();
	let mut terminal_safe_variants = Vec::new();

	for variant in &data.variants {
		let variant_name = &variant.ident;

		let handler_name = variant
			.attrs
			.iter()
			.find_map(|attr| {
				if attr.path().is_ident("handler") {
					attr.parse_args::<syn::Ident>().ok()
				} else {
					None
				}
			})
			.unwrap_or_else(|| variant_name.clone());

		let is_terminal_safe = variant
			.attrs
			.iter()
			.any(|attr| attr.path().is_ident("terminal_safe"));

		if is_terminal_safe {
			let pattern = match &variant.fields {
				syn::Fields::Unit => quote! { Self::#variant_name },
				syn::Fields::Unnamed(_) => quote! { Self::#variant_name(..) },
				syn::Fields::Named(_) => quote! { Self::#variant_name { .. } },
			};
			terminal_safe_variants.push(pattern);
		}

		let handler_screaming = to_screaming_snake_case(&handler_name.to_string());
		let slice_ident = format_ident!("RESULT_{}_HANDLERS", handler_screaming);

		if !slice_names.contains(&slice_ident) {
			slice_names.push(slice_ident.clone());
		}

		let pattern = match &variant.fields {
			syn::Fields::Unit => quote! { #enum_name::#variant_name },
			syn::Fields::Unnamed(_) => quote! { #enum_name::#variant_name(..) },
			syn::Fields::Named(_) => quote! { #enum_name::#variant_name { .. } },
		};

		match_arms.push(quote! {
			#pattern => run_handlers(&#slice_ident, result, ctx, extend)
		});
	}

	let expanded = quote! {
		#[allow(non_upper_case_globals)]
		mod __dispatch_result_slices {
			use super::*;
			use ::linkme::distributed_slice;
			use crate::editor_ctx::ResultHandler;

			#(
				#[distributed_slice]
				pub static #slice_names: [ResultHandler];
			)*
		}

		pub use __dispatch_result_slices::*;

		impl #enum_name {
			/// Returns true if this result can be applied when a terminal is focused.
			pub fn is_terminal_safe(&self) -> bool {
				matches!(self, #(#terminal_safe_variants)|*)
			}
		}

		/// Dispatches an action result to its registered handlers.
		///
		/// Returns `true` if the editor should quit.
		pub fn dispatch_result(
			result: &#enum_name,
			ctx: &mut crate::editor_ctx::EditorContext,
			extend: bool,
		) -> bool {
			use crate::editor_ctx::HandleOutcome;
			use crate::editor_ctx::MessageAccess;

			fn run_handlers(
				handlers: &[crate::editor_ctx::ResultHandler],
				result: &#enum_name,
				ctx: &mut crate::editor_ctx::EditorContext,
				extend: bool,
			) -> bool {
				for handler in handlers {
					match (handler.handle)(result, ctx, extend) {
						HandleOutcome::Handled => return false,
						HandleOutcome::Quit => return true,
						HandleOutcome::NotHandled => continue,
					}
				}
				ctx.notify(
					"info",
					&format!(
						"Unhandled action result: {:?}",
						::std::mem::discriminant(result)
					),
				);
				false
			}

			match result {
				#(#match_arms,)*
			}
		}
	};

	expanded.into()
}

pub(crate) fn to_screaming_snake_case(s: &str) -> String {
	let mut result = String::new();
	for (i, c) in s.chars().enumerate() {
		if c.is_uppercase() && i > 0 {
			result.push('_');
		}
		result.push(c.to_ascii_uppercase());
	}
	result
}
