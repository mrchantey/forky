use super::*;
use crate::node_system::parse_node_system;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::AttributeArgs;
use syn::ItemFn;
use syn::Visibility;

pub struct NodeParser {
	pub num_edges: usize,
	pub func: ItemFn,
	pub ident: Ident,
	pub vis: Visibility,
	pub edge_params: TokenStream,
	pub edge_bounds: TokenStream,
	// pub fields: TokenStream,
	// pub fields_typed: TokenStream,
	pub builder: NodePluginParser,
}

impl NodeParser {
	pub fn parse(
		attr: proc_macro::TokenStream,
		item: proc_macro::TokenStream,
	) -> proc_macro::TokenStream {
		let node_system: proc_macro2::TokenStream =
			parse_node_system(attr.clone(), item.clone()).into();

		let attr = parse_macro_input!(attr as AttributeArgs);
		let item = parse_macro_input!(item as ItemFn);
		let node = NodeParser::new(item, attr);

		let builder_impl = impl_builder(&node);
		let self_impl = impl_self(&node);
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
			#node_system
		}
		.into()
	}
	pub fn new(func: ItemFn, attr: AttributeArgs) -> Self {
		let num_edges = parse_attributes(attr).unwrap_or(2);
		let (edge_params, edge_bounds) = edge_generics(num_edges);
		let builder = NodePluginParser::new(&func, num_edges);
		Self {
			builder,
			num_edges,
			edge_params,
			edge_bounds,
			func: func.clone(),
			ident: func.sig.ident,
			vis: func.vis,
		}
	}
}

fn parse_attributes(attr: syn::AttributeArgs) -> Option<usize> {
	if let Some(first) = attr.first() {
		if let syn::NestedMeta::Lit(syn::Lit::Int(val)) = first {
			return val.base10_parse::<usize>().ok();
		}
	}
	return None;
}
