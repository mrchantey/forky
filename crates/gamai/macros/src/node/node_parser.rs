use super::*;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::AttributeArgs;
use syn::ItemStruct;
use syn::Visibility;

pub struct NodeParser {
	pub num_edges: usize,
	pub ident: Ident,
	pub vis: Visibility,
	pub edge_params: TokenStream,
	pub edge_bounds: TokenStream,
	// pub fields: TokenStream,
	// pub fields_typed: TokenStream,
	pub builder: NodePluginParser,
}

impl NodeParser {
	pub fn new(item: ItemStruct, attr: AttributeArgs) -> Self {
		let num_edges = parse_attributes(attr).unwrap_or(2);
		let (edge_generic_params, edge_generic_bounds) =
			edge_generics(num_edges);
		// let (fields, fields_typed) = fields(num_edges);
		let builder = NodePluginParser::new(&item, num_edges);
		Self {
			builder,
			num_edges,
			edge_params: edge_generic_params,
			edge_bounds: edge_generic_bounds,
			// fields,
			// fields_typed,
			ident: item.ident,
			vis: item.vis,
		}
	}
	pub fn parse(
		attr: proc_macro::TokenStream,
		item: proc_macro::TokenStream,
	) -> proc_macro::TokenStream {
		let attr = syn::parse_macro_input!(attr as syn::AttributeArgs);
		let item = syn::parse_macro_input!(item as syn::ItemStruct);
		let node = NodeParser::new(item, attr);

		let builder_impl = impl_builder(&node);
		let self_impl = impl_self(&node);
		let node_impl = impl_node(&node);
		let sets_impl = impl_sets(&node);
		let bundle_impl = impl_bundle(&node);

		quote! {
			use bevy::prelude::*;
			use gamai::*;
			#self_impl
			#sets_impl
			#builder_impl
			#node_impl
			#bundle_impl
		}
		.into()
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
