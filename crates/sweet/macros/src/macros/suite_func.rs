use super::*;
// use proc_macro2::Span;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;
use quote::TokenStreamExt;
use std::iter::Peekable;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;


pub struct SuiteFunc {
	pub out: proc_macro::TokenStream,
}

impl Parse for SuiteFunc {
	fn parse(stream: ParseStream) -> syn::parse::Result<Self> {
		let mut iter = into_peekable(stream)?;
		let mut stream = TokenStream::new();
		while let Some(t) = iter.next() {
			let t = parse_next(t, &mut iter)?;
			stream.append_all(t);
		}
		let out = stream.into();

		Ok(Self { out })
	}
}

fn parse_next(
	tree: TokenTree,
	iter: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> Result<TokenStream> {
	match tree {
		TokenTree::Ident(ident) => {
			let i_str = ident.to_string();
			match i_str.as_str() {
				"test" | "it" => {
					let case_flags = TestCaseFlags::parse(iter)?;
					if let Some(func) = iter.next() {
						match func {
							TokenTree::Group(func) => {
								Ok(parse_test_case(&func.stream(), &case_flags))
							}
							other => Err(syn::Error::new(
								ident.span(),
								&format!("Expected block, found {:?}", other),
							)),
						}
					} else {
						Err(syn::Error::new(ident.span(), "Expected block"))
					}
				}
				"before" => {
					todo!();
				}
				"after" => {
					todo!();
				}
				_ => Err(syn::Error::new(
					ident.span(),
					&format!("Expected \"test\" or \"it\", found ${:?}", ident),
				)),
			}
		}
		_ => Err(syn::Error::new(
			tree.span(),
			&format!("Expected \"test\" or \"it\", found ${:?}", tree),
		)),
	}
}
