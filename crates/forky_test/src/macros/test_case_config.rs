use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use std::iter::Peekable;

pub fn parse_case_config<I>(
	iter: &mut Peekable<I>,
) -> syn::parse::Result<TokenStream>
where
	I: Iterator<Item = TokenTree>,
{
	let mut skip = false;
	let mut only = false;
	let mut context = quote!(sweet::TestRunEnvironment::Unit);


	while let Some(t) = iter.peek() {
		match t {
			TokenTree::Ident(ident) => {
				let i_str = ident.to_string();
				match i_str.as_str() {
					"skip" => {
						let _ = iter.next().unwrap();
						skip = true;
					}
					"only" => {
						let _ = iter.next().unwrap();
						only = true;
					}
					"e2e" => {
						let _ = iter.next().unwrap();
						context =
							quote! {sweet::TestRunEnvironment::EndToEnd};
					}
					_ => {
						return Err(syn::parse::Error::new(
							ident.span(),
							"Config values: skip, only, e2e",
						));
					}
				}
			}
			_ => break,
		}
	}
	Ok(quote! {sweet::TestCaseConfig{
			skip:#skip,
			only:#only,
			context:#context,
		}
	})
}
