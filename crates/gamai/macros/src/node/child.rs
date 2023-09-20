use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;


// pub fn edge_field(index: usize) -> TokenStream {
// 	field_ident("edge", index).to_token_stream()
// }
pub fn child_type_param_name(index: usize) -> TokenStream {
	field_ident("Child", index).to_token_stream()
}

pub fn child_generics(num_params: usize) -> (TokenStream, TokenStream) {
	let params = (0..num_params)
		.map(|index| {
			let edge = child_type_param_name(index);
			quote!(#edge,)
		})
		.collect();

	let bounds = (0..num_params)
		.map(|index| {
			let edge = child_type_param_name(index);
			quote!(
				#edge:AiNode,
			)
		})
		.collect();

	(params, bounds)
}
