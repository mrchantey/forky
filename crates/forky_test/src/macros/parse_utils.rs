use proc_macro2::Literal;
use proc_macro2::TokenTree;
use std::iter::Peekable;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Result;

pub fn into_peekable(
	stream: ParseStream,
) -> Result<Peekable<impl Iterator<Item = TokenTree>>> {
	let stream = proc_macro2::TokenStream::parse(&stream)?;
	Ok(stream.into_iter().peekable())
}

pub fn parse_name<I>(iter: &mut Peekable<I>) -> Literal
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
pub fn try_remove_comma<I>(iter: &mut Peekable<I>) -> bool
where
	I: Iterator<Item = TokenTree>,
{
	if let Some(t) = iter.peek() {
		if let TokenTree::Punct(_punc) = t {
			let _ = iter.next().unwrap();
			return true;
		}
	};
	false
}

// pub fn parse_error(span: Span, msg: &str) -> TokenStream {
// 	Error::new(span, msg).to_compile_error()
// }

// pub fn parse_group<I>(func: &Ident, iter: &mut I) -> TokenStream
// where
// 	I: Iterator<Item = TokenTree>,
// {
// 	if let Some(t) = iter.next() {
// 		if let TokenTree::Group(l) = t {
// 			l.stream()
// 		} else {
// 			Error::new(func.span(), "unexpected identifier").to_compile_error()
// 		}
// 	} else {
// 		Error::new(func.span(), "unexpected token").to_compile_error()
// 	}
// }

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
