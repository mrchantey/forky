use std::iter::Peekable;

use proc_macro2::{Literal, Span, TokenStream, TokenTree};
use quote::*;
use syn::Ident;

use super::*;


pub struct SuiteParser {
	pub before: TokenStream,
	pub after: TokenStream,
	pub tests: Vec<TestParser>,
}

impl SuiteParser {
	pub fn parse<I>(iter: &mut Peekable<I>) -> proc_macro::TokenStream
	where
		I: Iterator<Item = TokenTree>,
	{
		let mut suite = SuiteParser {
			before: TokenStream::new(),
			after: TokenStream::new(),
			tests: Vec::new(),
		};
		let name = suite.parse_name(iter);
		let body = suite.parse_body(iter);

		build_suite(name, body)
	}

	fn parse_name<I>(&mut self, iter: &mut Peekable<I>) -> Literal
	where
		I: Iterator<Item = TokenTree>,
	{
		let mut name = Literal::string("undefined");
		if let Some(t) = iter.peek() {
			if let TokenTree::Literal(_lit) = t {
				let n = iter.next().unwrap();
				if let TokenTree::Literal(n) = n {
					name = n;
				}
			}
		};
		name
	}

	fn parse_body<I>(&mut self, iter: &mut I) -> TokenStream
	where
		I: Iterator<Item = TokenTree>,
	{
		let mut out = TokenStream::new();

		while let Some(t) = iter.next() {
			if !self.try_parse_custom_token(&t, iter) {
				out.append(t);
				// out.push(t);
			}
		}
		self.collect_tests(&mut out);
		out.append_all(quote! {Ok(())});
		out
	}

	fn collect_tests(&mut self, out: &mut TokenStream) {
		let test_ident = Ident::new("test", Span::call_site());
		let skip_ident = Ident::new("skip", Span::call_site());
		let before = &self.before;
		let after = &self.after;

		if self.tests.iter().filter(|t| t.only).next().is_some() {
			self.tests
				.iter_mut()
				.filter(|t| !t.only)
				.for_each(|t| t.skip = true);
		}

		for test in self.tests.iter() {
			let name = &test.name;
			let body = &test.body;
			let func = if test.skip { &skip_ident } else { &test_ident };
			out.append_all(quote! {
				#before
				s.#func(#name,||{
					#body
					Ok(())
				});
				#after
			});
		}
	}

	fn try_parse_custom_token<I>(
		&mut self,
		name: &TokenTree,
		iter: &mut I,
	) -> bool
	where
		I: Iterator<Item = TokenTree>,
	{
		match name {
			TokenTree::Ident(ident) => {
				let i_str = ident.to_string();
				let i_str = i_str.as_str();
				match i_str {
					"test" | "it" => {
						self.tests.push(TestParser::new(ident, iter));
						true
					}
					"before" => {
						self.before = parse_group(ident, iter);
						true
					}
					"after" => {
						self.after = parse_group(ident, iter);
						true
					}
					_ => false,
				}
			}
			_ => false,
		}
	}
}
