use super::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_self(node: &NodeParser) -> TokenStream {
	// let states_typed = get_states_typed(node);
	let NodeParser { ident, .. } = node;
	quote! {
		#[derive(Debug,Default,Clone)]
		#[allow(non_camel_case_types)]
		pub struct #ident;
	}
}
