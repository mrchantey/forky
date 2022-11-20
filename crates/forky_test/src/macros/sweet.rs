use super::*;
use syn::parse::{Parse, ParseStream, Result};

pub struct Sweet {
	pub out: proc_macro::TokenStream,
}

impl Parse for Sweet {
	fn parse(stream: ParseStream) -> Result<Self> {
		let stream = proc_macro2::TokenStream::parse(&stream)?;
		let mut stream = stream.into_iter().peekable();

		let out = SuiteParser::parse(&mut stream);

		Ok(Sweet { out })
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
