#![cfg_attr(
	debug_assertions,
	allow(dead_code, unused_imports, unused_variables, unused, unused_mut)
)]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::*;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, DeriveInput, Result, Token};

mod _macros;
use _macros::*;

#[proc_macro]
pub fn sweet(input: TokenStream) -> TokenStream { parse_macro_input!(input as Sweet).out }
