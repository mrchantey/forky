use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;


// pub fn choice_field(index: usize) -> TokenStream {
// 	field_ident("choice", index).to_token_stream()
// }
pub fn choice_type(index: usize) -> TokenStream {
	field_ident("Choice", index).to_token_stream()
}

pub fn choice_generics(num_params: usize) -> (TokenStream, TokenStream) {
	let params = (0..num_params)
		.map(|index| {
			let choice = choice_type(index);
			quote!(#choice,)
		})
		.collect();

	let bounds = (0..num_params)
		.map(|index| {
			let choice = choice_type(index);
			quote!(
				#choice:ChoiceSystems,
			)
		})
		.collect();

	(params, bounds)
}
