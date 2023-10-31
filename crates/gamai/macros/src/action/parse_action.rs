// use crate::*;
use super::*;
use crate::utils::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;
use syn::Result;

pub fn parse_action(
	attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> Result<TokenStream> {
	let func = syn::parse::<ItemFn>(item)?;
	let args = ActionArgs::from_tokens(attr.into())?;

	let ItemFn { vis, sig, .. } = func.clone();
	let ident = &sig.ident;

	let func_as_inner = func_as_inner(&func);
	let impl_into_action = impl_into_action(&func, &args);
	Ok(quote! {

		#[doc(hidden)]
		#func_as_inner
		use gamai::exports::*;
		use gamai::node::*;
		use gamai::prop::*;
		use gamai::*;

		#[derive(Debug, Clone, Eq, PartialEq, std::hash::Hash)]
		#[allow(non_camel_case_types)]
		#vis struct #ident;

		#impl_into_action
	})
}

// const GENERIC_ERROR:&str = "an `action` must have a single type parameter bound by `gamai::AiNode` ie: \npub fn my_func<Node: AiNode>()`";

fn impl_into_action(func: &ItemFn, args: &ActionArgs) -> TokenStream {
	let ident = &func.sig.ident;
	let func_inner = func_inner_ident(func);
	let ActionArgs { bundle, order } = args;
	// let generic_err = assert_single_generic_bound(
	// 	func.sig.generics.clone(),
	// 	"AiNode",
	// 	GENERIC_ERROR,
	// )
	// .unwrap_or_else(syn::Error::into_compile_error);

	let func_generic = if func_is_generic(&func) {
		quote!(::<Node>)
	} else {
		quote!()
	};

	quote! {
		impl IntoPropBundle for #ident
		{
			fn into_bundle<Node: AiNode>(self) -> impl Bundle { #bundle }
		}

		impl IntoAction for #ident
		{
			fn into_action_configs<Node: AiNode>(self) -> SystemConfigs{
				#func_inner #func_generic
					.in_set(ActionSet::new::<Node>(#order))
					.into_configs()
			}
		}
		// #generic_err
	}
}

fn func_is_generic(func: &ItemFn) -> bool {
	false == func.sig.generics.params.is_empty()
}

// use proc_macro2::Span;
// use proc_macro2::TokenStream;
// use syn::Generics;

// pub fn assert_single_generic_bound(
// 	generics: Generics,
// 	expected_bound:&str,
// 	err:&str,
// ) -> Result<TokenStream, syn::parse::Error> {
// 	if generics.params.len() == 1 {
// 		let param = generics.params.first().unwrap();
// 		if let syn::GenericParam::Type(param) = param {
// 			if param.bounds.len() == 1 {
// 				if let syn::TypeParamBound::Trait(bound) =
// 					param.bounds.first().unwrap()
// 				{
// 					if let Some(path) = bound.path.segments.last() {
// 						if path.ident.to_string().as_str() == expected_bound {
// 							return Ok(TokenStream::new());
// 							// return Ok(param.ident.clone().into_token_stream());
// 						}
// 					}
// 				}
// 			}
// 		}
// 	}
// 	return Err(syn::Error::new(Span::call_site(), err));
// }
