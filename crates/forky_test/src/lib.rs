mod macros;
use macros::*;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn sweet(input: TokenStream) -> TokenStream {
	parse_macro_input!(input as Sweet).out
}
