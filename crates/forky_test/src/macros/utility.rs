use proc_macro2::{Literal, Span, TokenStream, TokenTree};
use quote::*;
use syn::{Error, Ident};



// pub fn append_token_stream(out: &mut Vec<TokenTree>, stream: TokenStream) {
// 	let stream: proc_macro2::TokenStream = stream.into();
// 	for item in stream.into_iter() {
// 		out.push(item);
// 	}
// }

// pub fn make_error(span:Span,)

// pub fn throw(span: Span, msg: &str, vec: &mut Vec<TokenTree>) {
// 	let s = syn::Error::new(span, msg).to_compile_error();
// 	for i in s {
// 		vec.push(i);
// 	}
// }

pub fn parse_error(span: Span, msg: &str) -> TokenStream {
	Error::new(span, msg).to_compile_error()
}

pub fn parse_group<I>(func: &Ident, iter: &mut I) -> TokenStream
where
	I: Iterator<Item = TokenTree>,
{
	if let Some(t) = iter.next() {
		if let TokenTree::Group(l) = t {
			l.stream()
		} else {
			Error::new(func.span(), "unexpected identifier").to_compile_error()
		}
	} else {
		Error::new(func.span(), "unexpected token").to_compile_error()
	}
}


pub fn build_suite(
	name: Literal,
	body: TokenStream,
) -> proc_macro::TokenStream {
	quote! {
		use sweet::*;
		inventory::submit!(sweet::TestSuiteDesc {
			name: #name,
			func: |s|{
				#body
			},
			file: file!(),
		});
	}
	.into()
}
