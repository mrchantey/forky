use forky_core::StringX;
// use proc_macro::;
use proc_macro2::{Literal, Span, TokenStream, TokenTree};
use quote::*;
use syn::parse::{Parse, ParseStream, Result};
use syn::{braced, Attribute, GenericParam, Ident, Lifetime, Stmt, Token, Type, UseTree};

pub struct Sweet {
	pub out: proc_macro::TokenStream,
}

impl Parse for Sweet {
	fn parse(input: ParseStream) -> Result<Self> {
		let stream = proc_macro2::TokenStream::parse(&input)?;
		let mut iter = stream.into_iter().peekable();
		//TODO from file
		let mut name: Literal = Literal::string("my test");
		// let mut na
		if let Some(t) = iter.peek() {
			if let TokenTree::Literal(lit) = t {
				let n = iter.next().unwrap();
				if let TokenTree::Literal(n) = n {
					name = n;
				}
			}
		};

		let mut out = Vec::new();
		while let Some(t) = iter.next() {
			match t {
				TokenTree::Ident(ident) => {
					let i_str = ident.to_string();
					if i_str == "test" {
						parse_test(&mut iter, &mut out);
						continue;
					} else {
						out.push(ident.into())
					}
				}
				tt => out.push(tt),
			};
		}
		// let name = syn::LitStr::new(&name, Span::call_site());

		let body: proc_macro2::TokenStream = out.into_iter().collect();
		let out: proc_macro::TokenStream = quote! {
			use inventory::{submit,*};
			use forky_test::*;
			inventory::submit!(TestSuiteDesc {
				name: #name,
				func: |s|{
					#body
				},
				file: file!(),
			});
		}
		.into();

		Ok(Sweet { out })
	}
}
fn throw(span: Span, msg: &str, vec: &mut Vec<TokenTree>) {
	let s = syn::Error::new(span, msg).to_compile_error();
	for i in s {
		vec.push(i);
	}
}

fn parse_test<I>(iter: &mut I, out: &mut Vec<TokenTree>)
where
	I: Iterator<Item = TokenTree>,
{
	// let mut func: String = String::from("test");
	let mut func = "test";
	let mut name: Literal = Literal::string("foobar");
	// let mut name: String = String::from("fizzbar");
	let body: TokenTree = loop {
		if let Some(t) = iter.next() {
			match t {
				TokenTree::Group(l) => {
					break l.into();
				}
				TokenTree::Literal(l) => {
					name = l;
				}
				TokenTree::Ident(l) => {
					let s = l.to_string();
					if s == "skip" {
						func = "skip";
					} else if s == "only" {
						func = "only";
					} else {
						throw(l.span(), "unexpected identifier", out);
					}
				}
				tt => {
					throw(tt.span(), "unexpected token", out);
				}
			};
		}
	};

	// let name = syn::LitStr::new(&name[..], Span::call_site());
	let func = syn::Ident::new(&func, Span::call_site());
	// let name = "foobar";
	let func: proc_macro2::TokenStream = quote! {
			s.#func(#name,||{
				#body
				Ok(())
			})
	}
	.into();
	for item in func.into_iter() {
		out.push(item);
	}
}


// impl Parse for Root {
// 	fn parse(input: ParseStream) -> Result<Self> {
// 		let mut out = proc_macro2::TokenStream::new();
// 		while !input.is_empty() {
// 			let lookahead = input.lookahead1();

// 			if lookahead.peek(kw::test) {
// 				println!("woah!")
// 			} else if lookahead.peek(Ident) {
// 				let a = input.parse()?;
// 				input.parse().map(GenericParam::Type);
// 			} else if lookahead.peek(Lifetime) {
// 				input.parse().map(GenericParam::Lifetime);
// 			} else if lookahead.peek(Token![const]) {
// 				input.parse().map(GenericParam::Const);
// 			} else {
// 				Err(lookahead.error());
// 			}
// 		}

// 		let input = proc_macro2::TokenStream::parse(&input)?;
// 		let mut stream = input.into_iter();

// 		let out: TokenStream = out.into();
// 		Ok(Root { out })
// 	}
// 	// fn defaultParse =
// }
