use crate::field_ui_option;
use crate::parse_struct;
use crate::utils::*;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use std::collections::HashMap;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use syn::spanned::Spanned;
use syn::DataEnum;
use syn::DeriveInput;
use syn::Expr;
use syn::Field;
use syn::ItemStruct;
use syn::Result;
use syn::Variant;

pub fn parse_enum(input: DataEnum) -> Result<TokenStream> {
	let variants = input
		.variants
		.iter()
		.map(parse_enum_variant)
		.collect::<Result<Vec<_>>>()?
		.collect_comma_punct();

	Ok(quote! {
		let select = SelectField::new(
			reflect.field_name.clone(),
			reflect.clone_get_cb(),
			reflect.clone_set_cb(),
		);

		let val = reflect.get();
		#[allow(unused_variables)]
		match val{
			#variants
		}
	})
}


fn parse_enum_variant(variant: &Variant) -> Result<TokenStream> {
	let variant_ident = &variant.ident;
	let out = match &variant.fields {
		syn::Fields::Unit => {
			quote! {Self::#variant_ident => select.into()}
		}
		syn::Fields::Unnamed(fields) => {
			let field_idents = unnamed_field_idents(fields);
			let variant_with_fields =
				quote! {Self::#variant_ident(#field_idents)};
			let fields = fields
				.unnamed
				.iter()
				.enumerate()
				.map(|(i, field)| {
					let mut field = field.clone();
					field.ident = Some(field_ident(i, &field));
					parse_enum_field(&field, &variant_with_fields)
				})
				.collect::<Result<Vec<_>>>()?
				.collect_comma_punct();
			quote! {
				Self::#variant_ident(#field_idents) =>
				GroupField::new(reflect.display_name.clone(), vec![
				select.into(),
				#fields
			]).into()
			}
		}
		syn::Fields::Named(fields) => {
			let field_idents = fields
				.named
				.iter()
				.map(|f| f.ident.to_token_stream())
				.collect::<Vec<_>>()
				.collect_comma_punct();
			let variant_with_fields =
				quote! {Self::#variant_ident{#field_idents}};
			let fields = fields
				.named
				.iter()
				.map(|field| parse_enum_field(&field, &variant_with_fields))
				.collect::<Result<Vec<_>>>()?
				.collect_comma_punct();
			quote! {variant_with_fields =>
					GroupField::new(reflect.display_name.clone(), vec![
					select.into(),
					#fields
				]).into()
			}
		}
	};

	Ok(out)
}

fn unnamed_field_idents(fields: &syn::FieldsUnnamed) -> TokenStream {
	fields
		.unnamed
		.iter()
		.enumerate()
		.map(|(i, field)| field_ident(i, field).into_token_stream())
		.collect::<Vec<_>>()
		.collect_comma_punct()
}

fn field_ident(index: usize, field: &Field) -> Ident {
	Ident::new(&format!("field{index}"), field.span())
}

const ERROR: &str = "Unexpected enum variant, usually because UI was not recreated after the enum changed";
fn parse_enum_field(
	field: &Field,
	variant_with_fields: &TokenStream,
) -> Result<TokenStream> {
	let field_ident = field.ident.as_ref().expect("field must have an ident");
	let ident_str = field_ident.to_string();

	let reflect = quote! {
		{
			let checked_get = {
				let get_cb = reflect.clone_get_cb();
				move || match get_cb() {
					#[allow(unused_variables)]
					#variant_with_fields => #field_ident,
					_ => panic!(#ERROR),
				}
			};
			let checked_set = {
				let get_cb = reflect.clone_get_cb();
				let set_cb = reflect.clone_set_cb();
				move |val| match get_cb() {
					#[allow(unused_variables)]
					#variant_with_fields => {
						let #field_ident = val;
						set_cb(#variant_with_fields);
					},
					_ => panic!(#ERROR),
				}
			};
			FieldReflect::new(
				#ident_str.to_string(),
				checked_get,
				checked_set,
			)
		}
	};

	Ok(field_ui_option(field, &reflect)?)
}
