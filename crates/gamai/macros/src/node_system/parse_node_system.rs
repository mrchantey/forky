use super::*;
use quote::quote;
use syn::parse_macro_input;
use syn::ItemFn;


pub fn parse_node_system(
	_attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let func = parse_macro_input!(item as ItemFn);
	// let node = NodeParser::new(item, 0);
	let ItemFn { vis, sig, .. } = func.clone();
	let ident = &sig.ident;

	let original_func = parse_original_function(&func);
	let into_node_system_impl = impl_into_node_system(&func);
	quote! {
		use bevy_ecs::prelude::*;
		use gamai::*;

		#[derive(Debug,Default,Clone)]
		#[allow(non_camel_case_types)]
		#vis struct #ident;

		#original_func
		#into_node_system_impl
	}
	.into()
}
