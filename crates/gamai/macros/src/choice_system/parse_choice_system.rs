use crate::*;
use quote::quote;
// use quote::ToTokens;
use syn::parse_macro_input;
use syn::ItemFn;

const GENERIC_ERROR:&str = "a `choice_system` must have a single type parameter bound by `gamai::Choice` ie: \npub fn my_func<C: Choice>()`";

pub fn parse_choice_system(
	_attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let item = parse_macro_input!(item as ItemFn);

	let generic_err = assert_single_generic_bound(
		item.sig.generics.clone(),
		"Choice",
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

		impl AddChoiceSystem for #struct_ident {
			fn add_choice_system<C: Choice>(
				&self,
				app: &mut App,
				set: impl SystemSet,
			) {
				app.add_systems(Update, #func_ident::<C>.in_set(set));
			}
		}

		#vis fn #ident() -> impl AddChoiceSystem { #struct_ident }
		#generic_err
	}
	.into()
}
