#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::*;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, DeriveInput, Result, Token};

mod _macros;
use _macros::*;

#[proc_macro]
pub fn sweet(input: TokenStream) -> TokenStream {
	// let Sweet { name, stream } = parse_macro_input!(input as Sweet);
	let input = proc_macro2::TokenStream::from(input);

	let mut root = syn::parse2::<Root>(input).unwrap();

	
	
	quote! {
		// let name = #name;
		// #stream
	}
	.into()
}


#[proc_macro]
pub fn tn(input: TokenStream) -> TokenStream {
	// input
	let Ternary {
		// condition,
		// then_branch,
		// else_branch,
	} = parse_macro_input!(input as Ternary);

	// quote! {
	// 	if #condition {
	// 		#then_branch
	// 	} else {
	// 		#else_branch
	// 	}
	// }
	// .into()
	quote! {
		if true {
			true
		} else {
			false
		}
	}
	.into()
}
