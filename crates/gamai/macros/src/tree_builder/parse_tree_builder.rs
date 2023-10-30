use super::*;
use crate::utils::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

// currently very basic, maybe allow children etc in the future
pub fn parse_tree_builder(
	_attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> syn::Result<TokenStream> {
	let func = syn::parse::<ItemFn>(item)?;

	let inner_ident = func_inner_ident(&func);
	let mut func_inner = func_as_inner(&func);
	strip_input_attrs(&mut func_inner);

	if is_snake_case(func.sig.ident.to_string().as_str()) {
		return Err(syn::Error::new(
			func.sig.ident.span(),
			"tree builder function must be PascalCase",
		));
	}

	let ident = &func.sig.ident;
	let vis = &func.vis;
	let props_name = props_name(&ident);
	let props_list = props_list(&func);
	let props = parse_props(&func);
	Ok(quote! {

		#[allow(non_snake_case)]
		#vis fn #ident(props:#props_name)->impl TreeElement{

			#[allow(non_snake_case)]
			#func_inner

			let #props_name{#props_list} = props;
			#inner_ident(#props_list)
		}

		#props
	})
}

fn strip_input_attrs(func: &mut ItemFn) {
	func.sig.inputs.iter_mut().for_each(|input| match input {
		syn::FnArg::Receiver(_) => {}
		syn::FnArg::Typed(val) => {
			val.attrs.clear();
		}
	});
}

fn is_snake_case(val: &str) -> bool {
	val.chars().next().unwrap().is_lowercase()
}
