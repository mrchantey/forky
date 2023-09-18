use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;
use syn::ItemFn;

pub struct NodePluginParser {
	pub builder_ident: Ident,
	pub builder_params: TokenStream,
	pub builder_bounds: TokenStream,
}

impl NodePluginParser {
	pub fn new(item: &ItemFn, num_edges: usize) -> Self {
		let builder_ident =
			Ident::new(&format!("{}Plugin", item.sig.ident), item.sig.ident.span());

		let (builder_params, builder_bounds) = builder_params(num_edges);

		Self {
			builder_ident,
			builder_params,
			builder_bounds,
		}
	}
}

fn builder_params(num_params: usize) -> (TokenStream, TokenStream) {
	let (edge_params, edge_bounds) = edge_generics(num_params);
	let params = quote!(NodeSystem, #edge_params);
	let bounds = quote!(NodeSystem: AddAiNodeSystem, #edge_bounds);
	(params, bounds)
}


pub fn impl_builder(node: &NodeParser) -> TokenStream {
	let plugin_impl = impl_plugin(node);

	let NodeParser {
		edge_params,
		// vis,
		builder,
		..
	} = node;
	let NodePluginParser {
		builder_ident,
		builder_params,
		builder_bounds,
		..
	} = builder;

	quote! {
		// #[derive(Debug)]
		#[allow(non_camel_case_types)]
		pub struct #builder_ident<#builder_params> where #builder_bounds{
			node: NodeSystem,
			edges: (#edge_params),
		}
		impl<#builder_params> #builder_ident<#builder_params> where #builder_bounds{
			pub fn new(node: fn()->NodeSystem, edges: (#edge_params))->Self
				where #builder_bounds {
				#builder_ident{ node:node(), edges }
			}
	}
		#plugin_impl
	}
}
