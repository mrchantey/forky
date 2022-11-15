use proc_macro2::{Literal, TokenStream, TokenTree};
use syn::Ident;

use super::*;


pub struct TestParser {
	pub name: Literal,
	pub body: TokenStream,
	pub skip: bool,
	pub only: bool,
}


impl TestParser {
	pub fn new<I>(func: &Ident, iter: &mut I) -> TestParser
	where
		I: Iterator<Item = TokenTree>,
	{
		let mut parser = TestParser {
			name: Literal::string("foobar"),
			body: TokenStream::new(),
			skip: false,
			only: false,
		};

		while let Some(t) = iter.next() {
			match t {
				TokenTree::Group(l) => {
					parser.body = l.stream();
					break;
				}
				TokenTree::Literal(l) => {
					parser.name = l;
				}
				TokenTree::Ident(l) => {
					let s = l.to_string();
					if s == "skip" {
						parser.skip = true;
					} else if s == "only" {
						parser.only = true;
					} else {
						parser.body =
							parse_error(func.span(), "unexpected identifier");
						break;
					}
				}
				_ => {
					parser.body = parse_error(func.span(), "unexpected token");
					break;
				}
			};
		}
		parser
	}
}
