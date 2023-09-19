use super::*;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::Expr;
use syn::ItemFn;
use syn::Lit;
use syn::Meta;
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
			parse_node_system(item.clone()).into();
		let num_edges = get_num_edges(attr);
		let item = parse_macro_input!(item as ItemFn);
		let node = NodeParser::new(item, num_edges);

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
	pub fn new(func: ItemFn, num_edges: usize) -> Self {
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

pub fn get_num_edges(attr: proc_macro::TokenStream) -> usize {
	let attr: TokenStream = attr.into();
	let default_val = 2;
	if attr.is_empty() {
		return default_val;
	}
	let attr: syn::Attribute = syn::parse_quote! {#[#attr]};
	let val = match attr.meta {
		Meta::NameValue(kvp) => {
			if let Expr::Lit(val) = kvp.value {
				if let Lit::Int(val) = val.lit {
					val.base10_parse::<usize>().ok()
				} else {
					None
				}
			} else {
				None
			}
		}
		_ => None,
	};
	val.unwrap_or(default_val)
}
