use crate::utils::props_name;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::ItemFn;
use syn::PatType;
use syn::Type;

pub fn parse_props(func: &ItemFn) -> TokenStream {
	let vis = &func.vis;

	let inputs = inputs_to_props(func);
	let ident = props_name(&func.sig.ident);
	let impl_default = impl_default(func);

	quote! {
		#[derive(gamai::exports::typed_builder::TypedBuilder)]
		#vis struct #ident{
			#inputs
		}
		#impl_default
	}
}

pub fn inputs_to_props(func: &ItemFn) -> TokenStream {
	map_inputs(func, |val| {
		let name = &val.pat;
		let ty = &val.ty;
		let default_attr = parse_default_attr(val);
		let option_attr = parse_option_attr(val);

		quote! {
			#default_attr
			#option_attr
			pub #name: #ty,
		}
	})
}

fn impl_default(func: &ItemFn) -> Option<TokenStream> {
	if can_be_default(func) {
		let props = props_name(&func.sig.ident);
		Some(quote! {
			impl Default for #props {
				fn default() -> Self {
					Self::builder().build()
				}
			}
		})
	} else {
		None
	}
}

fn can_be_default(func: &ItemFn) -> bool {
	func.sig.inputs.iter().all(|input| match input {
		syn::FnArg::Receiver(_) => false,
		syn::FnArg::Typed(val) => {
			parse_option_attr(val).is_some()
				|| parse_default_attr(val).is_some()
		}
	})
}

pub fn props_list(func: &ItemFn) -> TokenStream {
	map_inputs(func, |val| {
		let name = &val.pat;
		quote! { #name,}
	})
}


fn map_inputs(
	func: &ItemFn,
	map: impl Fn(&PatType) -> TokenStream,
) -> TokenStream {
	func.sig
		.inputs
		.iter()
		.map(|input| match input {
			syn::FnArg::Receiver(receiver) => {
				syn::Error::new(receiver.span(), "'self' not allowed here")
					.into_compile_error()
					.into()
			}
			syn::FnArg::Typed(val) => map(val),
		})
		.collect::<Vec<_>>()
		.into_iter()
		.collect::<TokenStream>()
}

fn parse_option_attr(val: &PatType) -> Option<TokenStream> {
	match *val.ty.clone() {
		Type::Path(path) => {
			if let Some(next) = path.path.segments.first() {
				if next.ident == "Option" {
					return Some(quote!(#[builder(setter(strip_option))]));
				}
			}
		}
		_ => {}
	}
	None
}

fn parse_default_attr(val: &PatType) -> Option<TokenStream> {
	for attr in &val.attrs {
		match &attr.meta {
			// #[default = 3] my_value: usize
			syn::Meta::NameValue(name_value) => {
				if name_value.path.is_ident("default") {
					let value = &name_value.value;
					return Some(quote!(#[builder(default=#value)]));
				}
			}
			// #[default] my_value: usize
			syn::Meta::Path(path) => {
				if path.is_ident("default") {
					return Some(quote!(#[builder(default)]));
				}
			}
			_ => {}
		}
	}
	None
}
