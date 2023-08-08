use super::*;
use proc_macro2::Span;
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
					let config = parse_config(iter)?;
					let name = parse_name(iter);
					/*
					TODO assert that next token is a group
					TokenTree::Group(_) => {
						break;
					}
					tree => {
						return Err(syn::parse::Error::new(
							tree.span(),
							"Expected curly braces",
						));
					}
							*/
					let func = iter.next().unwrap();
					Ok(to_inventory_wrap_func(name, func.into(), config))
				}
				"before" => {
					todo!();
				}
				"after" => {
					todo!();
				}
				_ => Err(to_error(ident.span(), ident.to_string().as_str())),
			}
		}
		_ => Err(to_error(tree.span(), tree.to_string().as_str())),
	}
}

fn to_error(span: Span, actual: &str) -> syn::Error {
	syn::Error::new(
		span,
		format!("expected \"it\" or \"test\", found \"{actual}\""),
	)
}
