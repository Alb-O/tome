//! KDL keybinding parsing macro.

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Token, parse_macro_input};

use crate::dispatch::to_screaming_snake_case;

pub fn parse_keybindings(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as ParseKeybindingsInput);

	match generate_keybindings(&input.action_name, &input.kdl_str) {
		Ok(tokens) => tokens.into(),
		Err(e) => syn::Error::new(input.kdl_span, e).to_compile_error().into(),
	}
}

struct ParseKeybindingsInput {
	action_name: String,
	kdl_str: String,
	kdl_span: proc_macro2::Span,
}

impl Parse for ParseKeybindingsInput {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let action_ident: syn::Ident = input.parse()?;
		input.parse::<Token![,]>()?;
		let kdl_lit: syn::LitStr = input.parse()?;

		Ok(ParseKeybindingsInput {
			action_name: action_ident.to_string(),
			kdl_str: kdl_lit.value(),
			kdl_span: kdl_lit.span(),
		})
	}
}

fn generate_keybindings(
	action_name: &str,
	kdl_str: &str,
) -> Result<proc_macro2::TokenStream, String> {
	let doc: kdl::KdlDocument = kdl_str
		.parse()
		.map_err(|e: kdl::KdlError| format!("KDL parse error: {e}"))?;

	let mut statics = Vec::new();
	let action_upper = to_screaming_snake_case(action_name);

	for node in doc.nodes() {
		let mode_name = node.name().value();
		let mode_upper = mode_name.to_uppercase();

		let mode_variant = match mode_name {
			"normal" => quote! { Normal },
			"insert" => quote! { Insert },
			"goto" => quote! { Goto },
			"view" => quote! { View },
			"window" => quote! { Window },
			"match" => quote! { Match },
			"space" => quote! { Space },
			other => {
				return Err(format!(
					"Unknown mode: {other}. Valid modes: normal, insert, goto, view, window, match, space"
				));
			}
		};

		let slice_ident = format_ident!("KEYBINDINGS_{}", mode_upper);

		for (idx, entry) in node.entries().iter().enumerate() {
			if entry.name().is_some() {
				continue;
			}

			let Some(key_str) = entry.value().as_string() else {
				continue;
			};

			let parsed = evildoer_keymap_parser::parse(key_str)
				.map_err(|e| format!("Invalid key \"{key_str}\": {e}"))?;

			let key_tokens = node_to_key_tokens(&parsed)?;

			let static_ident = format_ident!("KB_{}_{}__{}", action_upper, mode_upper, idx);

			statics.push(quote! {
				#[allow(non_upper_case_globals)]
				#[::linkme::distributed_slice(evildoer_manifest::keybindings::#slice_ident)]
				static #static_ident: evildoer_manifest::keybindings::KeyBindingDef =
					evildoer_manifest::keybindings::KeyBindingDef {
						mode: evildoer_manifest::keybindings::BindingMode::#mode_variant,
						key: #key_tokens,
						action: #action_name,
						priority: 100,
					};
			});
		}
	}

	Ok(quote! { #(#statics)* })
}

fn node_to_key_tokens(
	node: &evildoer_keymap_parser::Node,
) -> Result<proc_macro2::TokenStream, String> {
	use evildoer_keymap_parser::{Key as ParserKey, Modifier};

	let code_tokens = match &node.key {
		ParserKey::Char(c) => quote! { evildoer_base::key::KeyCode::Char(#c) },
		ParserKey::F(n) => quote! { evildoer_base::key::KeyCode::F(#n) },
		ParserKey::Group(g) => {
			return Err(format!(
				"Key groups (@{g:?}) not supported in compile-time bindings"
			));
		}
		key => {
			let variant = format_ident!("{}", format!("{key:?}"));
			quote! { evildoer_base::key::KeyCode::#variant }
		}
	};

	let ctrl = node.modifiers & (Modifier::Ctrl as u8) != 0;
	let alt = node.modifiers & (Modifier::Alt as u8) != 0;
	let shift = node.modifiers & (Modifier::Shift as u8) != 0;

	Ok(quote! {
		evildoer_base::key::Key {
			code: #code_tokens,
			modifiers: evildoer_base::key::Modifiers { ctrl: #ctrl, alt: #alt, shift: #shift },
		}
	})
}
