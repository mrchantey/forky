use crate::*;
use quote::quote;
use syn::parse_macro_input;
use syn::ItemFn;

const GENERIC_ERROR:&str = "a `node_system` must have a single type parameter bound by `gamai::AiNode` ie: \npub fn my_func<A: AiNode>()`";

pub fn parse_node_system(
	_attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let item = parse_macro_input!(item as ItemFn);

	let generic_err = assert_single_generic_bound(
		item.sig.generics.clone(),
		"AiNode",
		GENERIC_ERROR,
	)
	.unwrap_or_else(syn::Error::into_compile_error);


	let func_ident = syn::Ident::new(
		&format!("{}_func", item.sig.ident),
		item.sig.ident.span(),
	);
	let mut item_inner = item.clone();
	item_inner.sig.ident = func_ident.clone();

	let struct_ident = syn::Ident::new(
		&format!("{}_struct", item.sig.ident),
		item.sig.ident.span(),
	);
	let ident = item.sig.ident;
	let vis = item.vis;

	quote! {
		#item_inner

		use bevy::prelude::*;
		use gamai::*;

		#[derive(Clone)]
		#[allow(non_camel_case_types)]
		pub struct #struct_ident;

		impl AddAiNodeSystem for #struct_ident {
			fn add_node_system<A: AiNode>(
				&self,
				app: &mut App,
				set: impl SystemSet,
			) {
				app.add_systems(Update, #func_ident::<A>.in_set(set));
			}
		}

		#vis fn #ident() -> impl AddAiNodeSystem { #struct_ident }
		#generic_err
	}
	.into()
}
