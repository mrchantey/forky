use crate::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::ItemFn;

pub fn parse_node_system(
	_attr: proc_macro::TokenStream,
	item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
	let func = parse_macro_input!(item as ItemFn);
	// let node = NodeParser::new(item, 0);
	let ItemFn { vis, sig, .. } = func.clone();
	let ident = &sig.ident;

	let func_as_inner = func_as_inner(&func);
	let impl_into_node_system = impl_into_node_system(&func);
	quote! {
		use bevy_ecs::prelude::*;
		use gamai::*;

		#[allow(non_camel_case_types)]
		#vis struct #ident;
		#func_as_inner
		#impl_into_node_system
	}
	.into()
}

const GENERIC_ERROR:&str = "a `node_system` must have a single type parameter bound by `gamai::AiNode` ie: \npub fn my_func<Node: AiNode>()`";

fn impl_into_node_system(func: &ItemFn) -> TokenStream {
	let ident = &func.sig.ident;
	let func_ident = func_inner_ident(&func.sig.ident);

	let generic_err = assert_single_generic_bound(
		func.sig.generics.clone(),
		"AiNode",
		GENERIC_ERROR,
	)
	.unwrap_or_else(syn::Error::into_compile_error);

	quote! {
		impl IntoNodeSystem<(#ident,IsNodeSystem)> for #ident
		{
			fn into_node_system<Node: AiNode>(
				self,
				schedule: &mut Schedule,
				set: impl SystemSet,
			) {
				schedule.add_systems(#func_ident::<Node>.in_set(set));
			}
		}
		#generic_err
	}
}
