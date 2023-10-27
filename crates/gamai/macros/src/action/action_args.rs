use crate::utils::attribute_args;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
// use syn::spanned::Spanned;
use syn::Result;

pub struct ActionArgs {
	pub bundle: TokenStream,
}

impl ActionArgs {
	pub fn from_tokens(tokens: TokenStream) -> Result<Self> {
		let args =
			attribute_args(tokens, Some(&["props", "components", "ordering"]))?;
		let mut props = quote::quote!(());
		let mut components = quote::quote!(());

		for (key, value) in args.iter() {
			match key.to_string().as_str() {
				"props" => {
					props = match value {
						syn::Expr::Tuple(tuple) => tuple
							.elems
							.iter()
							.map(|item| quote!(Prop::<_, Node>::new(#item), ))
							.collect::<TokenStream>(),
						item => quote!(Prop::<_, Node>::new(#item)),
						// _ => Err(syn::Error::new(
						// 	value.span(),
						// 	"Expected Tuple Expression, ie `(Foo, Bar)`",
						// )),
					};
				}
				"components" => {
					components = value.into_token_stream();
				}
				"ordering" => {
					todo!("ordering = before_parent")
				}
				_ => {}
			}
		}

		let bundle = quote!(((#props), (#components)));

		Ok(Self { bundle })
	}
}
