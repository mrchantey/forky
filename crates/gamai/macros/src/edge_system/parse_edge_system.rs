use crate::*;
use quote::quote;
// use quote::ToTokens;
use syn::parse_macro_input;
use syn::ItemFn;

const GENERIC_ERROR:&str = "an `edge_system` must have a single type parameter bound by `gamai::AiEdge` ie: \npub fn my_func<E: AiEdge>()`";

pub fn parse_edge_system(
	_attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let item = parse_macro_input!(item as ItemFn);

	let generic_err = assert_single_generic_bound(
		item.sig.generics.clone(),
		"AiEdge",
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

		use bevy_ecs::prelude::*;
		use gamai::*;

		#[derive(Clone)]
		#[allow(non_camel_case_types)]
		pub struct #struct_ident;

		impl EdgeSystemBuilder for #struct_ident {
			fn add_edge_system<E: AiEdge>(
				&self,
				schedule: &mut Schedule,
				set: impl SystemSet,
			) {
				schedule.add_systems(#func_ident::<E>.in_set(set));
			}
		}

		#vis fn #ident() -> impl EdgeSystemBuilder { #struct_ident }
		#generic_err
	}
	.into()
}
