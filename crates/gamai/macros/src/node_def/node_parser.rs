use super::*;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::LitInt;

pub struct NodeParser {
	pub num_edges: usize,
	pub ident: Ident,
	pub self_params: TokenStream,
	pub self_params_types_only: TokenStream,
	pub self_bounds: TokenStream,
	pub child_params: TokenStream,
	pub child_bounds: TokenStream,
}

impl NodeParser {
	pub fn parse_node(
		tokens: proc_macro::TokenStream,
	) -> proc_macro::TokenStream {
		let num_edges = get_num_edges(tokens).unwrap();
		let node = NodeParser::new(num_edges);

		let self_impl = impl_self(&node);
		let node_impl = impl_node(&node);

		quote! {
			use bevy_ecs::prelude::*;
			use gamai::*;
			#self_impl

			// #sets_impl
			#node_impl
			// #plugin_impl
			// #bundle_impl
		}
		.into()
	}
	pub fn new(num_edges: usize) -> Self {
		let ident = Ident::new(&format!("Node{num_edges}"), Span::call_site());
		let (child_params, child_bounds) = child_generics(num_edges);
		let self_params_types_only = quote!(NodeSystem, EdgeSystem,);
		let self_params = quote!(
			NodeSystem,
			EdgeSystem,
			NODE_ID,
			GRAPH_ID,
			GRAPH_DEPTH,
			CHILD_INDEX,
			PARENT_DEPTH,
			#child_params
		);
		let self_bounds = quote!(
			NodeSystem: IntoNodeSystem,
			EdgeSystem: IntoNodeSystem,
			const NODE_ID:usize,
			const GRAPH_ID:usize,
			const GRAPH_DEPTH:usize,
			const CHILD_INDEX: usize,
			const PARENT_DEPTH: usize,
			#child_bounds
		);

		Self {
			num_edges,
			self_params,
			self_params_types_only,
			self_bounds,
			child_params,
			child_bounds,
			ident,
		}
	}
}

pub fn get_num_edges(attr: proc_macro::TokenStream) -> syn::Result<usize> {
	if let Ok(lit) = syn::parse::<LitInt>(attr.into()) {
		let val = lit.base10_parse::<usize>()?;
		if val <= 117 {
			Ok(val)
		} else {
			Err(syn::Error::new(
				lit.span(),
				"maximum number of edges is 117",
			))
		}
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