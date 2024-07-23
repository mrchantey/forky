#![feature(proc_macro_hygiene)]
mod macros;
use macros::*;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn default_attrs_get(attr: TokenStream, input: TokenStream) -> TokenStream {
	parse_default_attr_get(attr, input)
		.unwrap_or_else(syn::Error::into_compile_error)
		.into()
}
#[proc_macro]
pub fn default_attrs_set(_input: TokenStream) -> TokenStream {
	quote!(<div>hi there</div>).into()
}
#[proc_macro_attribute]
pub fn experiment(_attr: TokenStream, _item: TokenStream) -> TokenStream {
	// item
	quote! {a:u32}.into()
}
