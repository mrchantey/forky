use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

/// returns childN
pub fn child_field_name(index: usize) -> TokenStream {
	field_ident("child", index).to_token_stream()
}

/// Returns ChildN
pub fn child_type_name(index: usize) -> TokenStream {
	field_ident("Child", index).to_token_stream()
}

pub fn child_fields_def(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_name(index);
			let ty = child_type_name(index);
			quote!(#field: #ty,)
		})
		.collect()
}
