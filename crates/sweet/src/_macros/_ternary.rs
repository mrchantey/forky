//https://github.com/zfzackfrost/iffy-rs/blob/main/src/lib.rs

extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Token};

type ParseResult<T> = syn::parse::Result<T>;

pub struct Ternary {
	// pub condition: Expr,
	// pub then_branch: Expr,
	// pub else_branch: Expr,
}

impl Parse for Ternary {
	fn parse(input: ParseStream) -> ParseResult<Self> {
		let condition = syn::Lit::parse(input)?;
		// match condition {
		// 	Expr::
		// }
		// input.parse::<Token![?]>()?;
		// let then_branch: Expr = input.parse()?;
		// input.parse::<Token![:]>()?;
		// let else_branch: Expr = input.parse()?;
		// let then_branch = syn::Expr:
		Ok(Ternary {
			// condition,
			// then_branch,
			// else_branch,
		})
	}
}
