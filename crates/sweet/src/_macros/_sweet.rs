//https://github.com/zfzackfrost/iffy-rs/blob/main/src/lib.rs

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Token, Attribute};

type ParseResult<T> = syn::parse::Result<T>;

mod keyword {
	use syn::custom_keyword;

	custom_keyword!(before);
	custom_keyword!(after);

	// Are aliases for eachother:
	custom_keyword!(describe);

	// Are aliases for eachother:
	custom_keyword!(it);
	custom_keyword!(test);
}

pub struct Sweet {
	pub name: syn::LitStr,
	pub stream: proc_macro2::TokenStream,
}

impl Parse for Sweet {
	fn parse(input: ParseStream) -> ParseResult<Self> {
		let has_name = input.peek(syn::LitStr);
		let name: syn::LitStr = if has_name {
			input.parse()?
		} else {
			syn::LitStr::new("foobar", input.span())
		};


		let fork = input.fork();
		let _attibutes = fork.call(Attribute::parse_outer)?;
		let _async_token = fork.parse::<Option<Token![async]>>()?;



		// let fork = 


		// if let

		// let condition = syn::Lit::parse(input)?;
		// match condition {
		// 	Expr::
		// }




		// let content;
		// let _braces = syn::braced!(content in input);
		let stream = proc_macro2::TokenStream::parse(input)?;



		// println!("{}",condition);
		// input.parse::<Token![?]>()?;
		// let then_branch: Expr = input.parse()?;
		// input.parse::<Token![:]>()?;
		// let else_branch: Expr = input.parse()?;
		// let then_branch = syn::Expr:
		Ok(Sweet {
			name,
			stream, // block,
			        // then_branch,
			        // else_branch,
		})
	}
}
