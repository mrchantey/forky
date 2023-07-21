use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use std::iter::Peekable;

pub fn parse_config<I>(iter: &mut Peekable<I>) -> TokenStream
where
	I: Iterator<Item = TokenTree>,
{
	if let Some(t) = iter.peek() {
		match t {
			TokenTree::Ident(ident) => {
				let i_str = ident.to_string();
				match i_str.as_str() {
					"skip" => {
						let _ = iter.next().unwrap();
						return quote! {sweet::TestCaseConfig::Skip};
					}
					"only" => {
						let _ = iter.next().unwrap();
						return quote! {sweet::TestCaseConfig::Only};
					}
					_ => {}
				}
			}
			_ => {}
		}
	};
	quote! {sweet::TestCaseConfig::Default}
}
