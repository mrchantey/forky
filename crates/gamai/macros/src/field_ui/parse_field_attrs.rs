use crate::utils::*;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
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
		None => quote! {#ty::into_field_ui(#reflect)},
		Some(FieldAttrs { ident, fields }) => {
			quote! {
				#ident::<#ty>{
					reflect: #reflect,
					#fields,
					..Default::default()
				}.into()
			}
		}
	};
	Ok(out)
}


struct FieldAttrs {
	ident: Ident,
	fields: TokenStream,
}
impl FieldAttrs {
	pub fn new(
		ident: &'static str,
		field_names: &[&'static str],
		args: HashMap<String, Option<Expr>>,
	) -> Self {
		let ident = Ident::new(ident, Span::call_site());
		let fields = field_names
			.iter()
			.filter_map(|name| {
				args.get(*name).and_then(|val| val.clone()).map(|val| {
					let ident = Ident::new(name, Span::call_site());
					quote!(#ident: #val)
				})
			})
			.collect::<Vec<_>>()
			.collect_comma_punct();
		Self { ident, fields }
	}
}

fn parse_field_attrs(field: &Field) -> Result<Option<FieldAttrs>> {
	for attr in field.attrs.iter() {
		let args: TokenStream = attr.parse_args()?;
		let args = attributes_map(args, None)?;

		if attr
			.meta
			.path()
			.is_ident(&Ident::new("slider", Span::call_site()))
		{
			panic!("deprecated");
		} else if attr
			.meta
			.path()
			.is_ident(&Ident::new("number", Span::call_site()))
		{
			return Ok(Some(FieldAttrs::new(
				"NumberField",
				&["min", "max", "step", "display"],
				args,
			)));
		}
	}
	Ok(None)
}
// .ok_or_else(|| {
// 	syn::Error::new(
// 		attr.span(),
// 		format!(
// 			"{attr_name} attribute must have a '{name}' arg"
// 		),
// 	)
// })?
// .clone()
// .ok_or_else(|| {
// 	syn::Error::new(
// 		attr.span(),
// 		format!(
// 			"{attr_name} attribute must have a '{name}' arg"
// 		),
// 	)
// })?)
