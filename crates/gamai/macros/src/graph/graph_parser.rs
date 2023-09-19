use proc_macro2::TokenStream;
// use super::*;
// use proc_macro2::Ident;
// use proc_macro2::TokenStream;
use quote::quote;
// use syn::parse_macro_input;
// use syn::AttributeArgs;
// use syn::ItemFn;
// use syn::Visibility;
// use rstml::node::Node;
// use rstml::node::NodeAttribute;

pub struct GraphParser;

impl GraphParser {
	pub fn parse(
		_item: proc_macro::TokenStream,
	) -> syn::Result<TokenStream> {
		// let nodes = rstml::parse2(item.into())
		// 	.map_err(syn::Error::into_compile_error)
		// 	.unwrap();
		// nodes.er
		Ok(quote!(32))
	}
}
