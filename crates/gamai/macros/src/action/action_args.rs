use crate::utils::attribute_args;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
// use syn::spanned::Spanned;
use syn::Result;

pub struct ActionArgs {
	pub order: TokenStream,
	pub props: TokenStream,
	pub components: TokenStream,
	pub apply_deferred: bool,
}

impl Default for ActionArgs {
	fn default() -> Self {
		Self {
			order: quote!(ActionOrder::Update),
			props: quote!(()),
			components: quote!(()),
			apply_deferred: false,
		}
	}
}

impl ActionArgs {
	pub fn from_tokens(tokens: TokenStream) -> Result<Self> {
		let args = attribute_args(
			tokens,
			Some(&["props", "components", "order", "apply_deferred"]),
		)?;
		let mut action_args = ActionArgs::default();

		for (key, value) in args.iter() {
			match key.to_string().as_str() {
				"props" => {
					action_args.props = match value {
						Some(syn::Expr::Tuple(tuple)) => tuple
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
				"apply_deferred" => {
					action_args.apply_deferred = true;
				}
				"components" => {
					action_args.components = value.into_token_stream();
				}
				"order" => {
					action_args.order = value.into_token_stream();
				}
				name => {
					return Err(syn::Error::new(
						key.span(),
						format!("Unknown attribute key: {name}"),
					))
				}
			}
		}
		Ok(action_args)
	}
}
