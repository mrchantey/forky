use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;
use syn::Result;


pub fn parse_default_attr_get(
	_attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> Result<TokenStream> {
	let mut function = syn::parse::<ItemFn>(item)?;
	// function.attrs.push(syn::parse_quote! {#[component]});
	function.sig.inputs.push(syn::parse_quote! {
		// #[prop(default)]
		param1: i32
	});

	Ok(quote! {
		#[component]
		#function
	})
}
