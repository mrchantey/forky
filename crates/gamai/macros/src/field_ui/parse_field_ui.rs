use crate::parse_enum;
use crate::parse_struct;
use crate::utils::*;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use syn::DeriveInput;
use syn::Expr;
use syn::Field;
use syn::ItemStruct;
use syn::Result;
use syn::Variant;

pub fn parse_field_ui(item: proc_macro::TokenStream) -> Result<TokenStream> {
	let input = syn::parse::<DeriveInput>(item)?;
	let ident = &input.ident;

	let out: TokenStream = match input.data {
		syn::Data::Struct(input) => parse_struct(input)?,
		syn::Data::Enum(input) => parse_enum(input)?,
		syn::Data::Union(_) => unimplemented!(),
	};


	Ok(quote! {
		impl IntoFieldUi for #ident{
			fn into_field_ui(reflect: FieldReflect<Self>) -> FieldUi {
				#out
			}
		}
	})
}
