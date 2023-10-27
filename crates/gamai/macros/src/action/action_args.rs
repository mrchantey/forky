use crate::utils::attribute_args;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Result;

pub struct ActionArgs {
	pub props: TokenStream,
}

impl ActionArgs {
	pub fn from_tokens(tokens: TokenStream) -> Result<Self> {
		let args = attribute_args(tokens, Some(&["props"]))?;
		let mut props = quote::quote!(());

		for (key, value) in args.iter() {
			match key.to_string().as_str() {
				"props" => {
					props = value.into_token_stream();
				}
				_ => {}
			}
		}


		Ok(Self { props })
	}
}
