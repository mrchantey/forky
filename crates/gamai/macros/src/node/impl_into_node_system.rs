use crate::*;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

const GENERIC_ERROR:&str = "a `node_system` must have a single type parameter bound by `gamai::AiNode` ie: \npub fn my_func<A: AiNode>()`";


pub fn impl_into_node_system(func: &ItemFn) -> TokenStream {
	let ident = &func.sig.ident;
	let add_node_system = parse_node_system(&func);
	quote! {
		impl IntoNodeSystem for #ident{
			#add_node_system
		}
	}
}

fn parse_node_system(func: &ItemFn) -> TokenStream {
	let generic_err = assert_single_generic_bound(
		func.sig.generics.clone(),
		"AiNode",
		GENERIC_ERROR,
	)
	.unwrap_or_else(syn::Error::into_compile_error);


	let func_ident = func_ident(&func.sig.ident);
	quote! {
			fn add_node_system<A: AiNode>(
				schedule: &mut Schedule,
				set: impl SystemSet,
			) {
				schedule.add_systems(#func_ident::<A>.in_set(set));
			}
		// }
		#generic_err
	}
}

pub fn func_ident(ident: &Ident) -> syn::Ident {
	syn::Ident::new(&format!("{}_func", ident), ident.span())
}

pub fn parse_original_function(func: &ItemFn) -> ItemFn {
	let mut func_inner = func.clone();
	func_inner.sig.ident = func_ident(&func.sig.ident);
	func_inner
}
