use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

pub fn child_field_param_name(index: usize) -> TokenStream {
	field_ident("child", index).to_token_stream()
}

/// Returns ChildN
pub fn child_type_param_name(index: usize) -> TokenStream {
	field_ident("Child", index).to_token_stream()
}

/// returns (Child0,(Child1,..))
pub fn _child_params_nested(node: &NodeParser) -> TokenStream {
	(0..node.num_edges)
		// .rev()
		.fold(TokenStream::new(), |prev, index| {
			let ident = child_type_param_name(index);
			quote!((#ident, #prev))
		})
		.into_token_stream()
}
/// returns (AiBundle<Child0>,(AiBundle<Child1>,..))
pub fn child_bundles_nested(node: &NodeParser) -> TokenStream {
	(0..node.num_edges)
		// .rev()
		.fold(TokenStream::new(), |prev, index| {
			let ident = child_type_param_name(index);
			quote!((AiBundle<#ident>, #prev))
		})
		.into_token_stream()
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

/// returns `child0: Child0, child1: Child1, ..`
pub fn child_fields_def(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_param_name(index);
			let ty = child_type_param_name(index);
			quote!(#field: #ty,)
		})
		.collect()
}
// returns `child0, child1, ..`
pub fn child_fields(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_param_name(index);
			quote!(#field,)
		})
		.collect()
}
