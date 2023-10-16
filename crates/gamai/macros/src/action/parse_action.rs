// use crate::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::Ident;
use syn::ItemFn;


pub fn parse_action(
	_attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let func = parse_macro_input!(item as ItemFn);
	// let node = NodeParser::new(item, 0);
	let ItemFn { vis, sig, .. } = func.clone();
	let ident = &sig.ident;

	let func_as_inner = func_as_inner(&func);
	let impl_into_action = impl_into_action(&func);
	quote! {
		#func_as_inner
		use gamai::exports::*;
		use gamai::*;

		#[derive(Debug, Default, Clone, Eq, PartialEq, std::hash::Hash)]
		#[allow(non_camel_case_types)]
		#vis struct #ident;

		#impl_into_action
	}
	.into()
}

// const GENERIC_ERROR:&str = "a `action` must have a single type parameter bound by `gamai::AiNode` ie: \npub fn my_func<Node: AiNode>()`";

fn impl_into_action(func: &ItemFn) -> TokenStream {
	let ident = &func.sig.ident;
	let func_inner = func_inner_ident(&func.sig.ident);

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
		impl IntoAction for #ident
		{
			fn into_action_configs<Node: AiNode>(self) -> SystemConfigs{
				#func_inner #func_generic.into_configs()
			}
		}
		// #generic_err
	}
}

fn func_is_generic(func: &ItemFn) -> bool {
	false == func.sig.generics.params.is_empty()
}

fn func_inner_ident(ident: &Ident) -> Ident {
	Ident::new(&format!("{}_inner", ident), ident.span())
}

fn func_as_inner(func: &ItemFn) -> ItemFn {
	let mut func_inner = func.clone();
	func_inner.sig.ident = func_inner_ident(&func.sig.ident);
	func_inner
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
