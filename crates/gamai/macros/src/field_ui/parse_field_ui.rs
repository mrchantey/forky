use crate::utils::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use syn::Expr;
use syn::ItemStruct;
use syn::Result;


pub fn parse_field_ui(
	_attr: proc_macro::TokenStream,
	_item: proc_macro::TokenStream,
) -> Result<TokenStream> {
	// let mut input = syn::parse::<ItemStruct>(item)?;
	// let args = &attributes_map(attr.into(), Some(&["system"]))?;

	// let action_trait = action_trait(&input, args);

	// remove_field_attributes(&mut input);

	Ok(quote! {
		// #input

		impl FooBar for BB{


		}
		// use gamai::prelude::*;
		// use gamai::exports::*;
		// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Component)]
		// #input
		// #action_trait
	})
}
