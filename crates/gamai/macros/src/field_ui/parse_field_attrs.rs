use crate::utils::*;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::Expr;
use syn::Field;
use syn::Ident;
use syn::Result;


pub fn field_ui_option(
	field: &Field,
	reflect: &TokenStream,
) -> Result<TokenStream> {
	let ty = &field.ty;

	let out = match parse_field_attrs(field)? {
		FieldAttrs::None => quote! {#ty::into_field_ui(#reflect)},
		FieldAttrs::Slider { min, max, step } => {
			quote! {
				SliderField::from_reflect(
					#reflect,
					#min,
					#max,
					#step,
				).into()
			}
		}
		FieldAttrs::Number { step } => {
			quote! {
				NumberField::from_reflect(
					#reflect,
					#step,
				).into()
			}
		}
	};

	Ok(out)
}


enum FieldAttrs {
	None,
	Slider { min: Expr, max: Expr, step: Expr },
	Number { step: Expr },
}

fn parse_field_attrs(field: &Field) -> Result<FieldAttrs> {
	for attr in field.attrs.iter() {
		let args: TokenStream = attr.parse_args()?;
		let args = attributes_map(args, None)?;
		let get = |attr_name: &str, name: &str| -> Result<Expr> {
			Ok(args
				.get(name)
				.ok_or_else(|| {
					syn::Error::new(
						attr.span(),
						format!(
							"{attr_name} attribute must have a '{name}' arg"
						),
					)
				})?
				.clone()
				.ok_or_else(|| {
					syn::Error::new(
						attr.span(),
						format!(
							"{attr_name} attribute must have a '{name}' arg"
						),
					)
				})?)
		};

		if attr
			.meta
			.path()
			.is_ident(&Ident::new("slider", Span::call_site()))
		{
			return Ok(FieldAttrs::Slider {
				min: get("slider", "min")?,
				max: get("slider", "max")?,
				step: get("slider", "step")?,
			});
		} else if attr
			.meta
			.path()
			.is_ident(&Ident::new("number", Span::call_site()))
		{
			return Ok(FieldAttrs::Number {
				step: get("number", "step")?,
			});
		}
	}
	Ok(FieldAttrs::None)
}
