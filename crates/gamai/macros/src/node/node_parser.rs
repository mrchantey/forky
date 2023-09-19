use super::*;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::ItemFn;
use syn::LitInt;

pub struct NodeParser {
	pub num_edges: usize,
	pub ident: Ident,
	pub edge_params: TokenStream,
	pub edge_bounds: TokenStream,
	pub builder: NodePluginParser,
}





impl NodeParser {
	pub fn parse_node_system(
		_attr: proc_macro::TokenStream,
		item: proc_macro::TokenStream,
	) -> proc_macro::TokenStream {
		let func = parse_macro_input!(item as ItemFn);
		// let node = NodeParser::new(item, 0);
		let ItemFn { vis, sig, .. } = func.clone();
		let ident = &sig.ident;

		let original_func = parse_original_function(&func);
		let into_node_system_impl = impl_into_node_system(&func);
		quote! {
			use bevy_ecs::prelude::*;
			use gamai::*;

			#[derive(Debug,Default,Clone)]
			#[allow(non_camel_case_types)]
			#vis struct #ident;

			#original_func
			#into_node_system_impl
		}
		.into()
	}


	pub fn parse_node(
		attr: proc_macro::TokenStream,
	) -> proc_macro::TokenStream {
		let num_edges = get_num_edges(attr).unwrap();
		let node = NodeParser::new(num_edges);

		let self_impl = impl_self(&node);
		// let original_func = parse_original_function(&node.func);
		// let into_node_system_impl = impl_into_node_system(&node);

		let builder_impl = impl_builder(&node);
		let node_impl = impl_node(&node);
		let sets_impl = impl_sets(&node);
		let bundle_impl = impl_bundle(&node);

		quote! {
			use bevy_ecs::prelude::*;
			use gamai::*;
			#self_impl

			#sets_impl
			#builder_impl
			#node_impl
			#bundle_impl
		}
		.into()
	}
	pub fn new(num_edges: usize) -> Self {
		let ident = Ident::new(&format!("Node{num_edges}"), Span::call_site());
		let (edge_params, edge_bounds) = edge_generics(num_edges);


		let builder = NodePluginParser::new(&ident, num_edges);
		Self {
			builder,
			num_edges,
			edge_params,
			edge_bounds,
			ident,
		}
	}
}

pub fn get_num_edges(attr: proc_macro::TokenStream) -> syn::Result<usize> {
	if let Ok(val) = syn::parse::<LitInt>(attr.into()) {
		Ok(val.base10_parse::<usize>()?)
	} else {
		Err(syn::Error::new(
			Span::call_site(),
			"please specify number of edges",
		))
	}

	// let attr: TokenStream = attr.into();
	// if attr.is_empty() {
	// 	return Err(syn::Error::new(
	// 		Span::call_site(),
	// 		"please specify number of edges",
	// 	));
	// }
	// let attr: syn::Attribute = syn::parse_quote! {#[#attr]};
	// match attr.meta {
	// 	Meta::NameValue(kvp) => {
	// 		if let Expr::Lit(val) = kvp.value {
	// 			if let Lit::Int(val) = val.lit {
	// 				val.base10_parse::<usize>()
	// 			} else {
	// 				Err(syn::Error::new(
	// 					val.lit.span(),
	// 					"please specify number of edges",
	// 				))
	// 			}
	// 		} else {
	// 			Err(syn::Error::new(
	// 				Span::call_site(),
	// 				"please specify number of edges",
	// 			))
	// 		}
	// 	}
	// 	_ => Err(syn::Error::new(
	// 		Span::call_site(),
	// 		"please specify number of edges",
	// 	)),
	// }
}
