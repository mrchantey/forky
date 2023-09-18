use super::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_self(node: &AiNode) -> TokenStream {
	// let states_typed = get_states_typed(node);
	let AiNode { ident, vis, .. } = node;
	quote! {
		#[derive(Debug,Default)]
		#vis struct #ident;
	}
}
