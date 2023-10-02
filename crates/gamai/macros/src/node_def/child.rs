use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

pub fn child_field_param_name(index: usize) -> TokenStream {
	field_ident("child", index).to_token_stream()
}
pub fn child_type_param_name(index: usize) -> TokenStream {
	field_ident("Child", index).to_token_stream()
}

pub fn child_generics(num_children: usize) -> (TokenStream, TokenStream) {
	let params = (0..num_children)
		.map(|index| {
			let ty = child_type_param_name(index);
			quote!(#ty,)
		})
		.collect();

	let bounds = (0..num_children)
		.map(|index| {
			let ty = child_type_param_name(index);
			quote!(
				#ty:AiNode,
			)
		})
		.collect();

	(params, bounds)
}

pub fn child_fields(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_param_name(index);
			let ty = child_type_param_name(index);
			quote!(#field: #ty,)
		})
		.collect()
}
