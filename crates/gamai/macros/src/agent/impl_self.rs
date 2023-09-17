use super::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_self(agent: &Agent) -> TokenStream {
	// let states_typed = get_states_typed(agent);
	let Agent { ident, vis, .. } = agent;
	quote! {
		#[derive(Debug,Default)]
		#vis struct #ident;
	}
}
