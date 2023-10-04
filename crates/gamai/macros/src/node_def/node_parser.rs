use super::*;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::LitInt;

pub struct NodeParser {
	pub num_edges: usize,
	/// The name of this node: `Node0`
	pub ident: Ident,
	/// all generic params for this type: `NodeSystem, EdgeSystem, NODE_ID, GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, Child1, Child2`
	pub self_params: TokenStream,
	/// systems-only generic params for this type: `NodeSystem, EdgeSystem`
	pub phantom_types: TokenStream,
	/// all generic bounds for this type: `NodeSystem: IntoNodeSystem, ...`
	pub self_bounds: TokenStream,
	/// types of the children: `Child1,Child2`
	pub child_params: TokenStream,
	/// bound types of the children: `Child1: AiNode, Child2: AiNode`
	pub child_bounds: TokenStream,

	pub self_params_new: TokenStream,
	pub self_bounds_full: TokenStream,
}


impl NodeParser {
	pub fn new(num_edges: usize) -> Self {
		let ident = Ident::new(&format!("Node{num_edges}"), Span::call_site());
		let (child_params, child_bounds) = child_generics(num_edges);
		let phantom_types = quote!(Parent, NodeSystemMarker, EdgeSystemMarker);

		let self_params = quote! {
			CHILD_INDEX,
			Parent,
			NodeSystem,
			NodeSystemMarker,
			EdgeSystem,
			EdgeSystemMarker,
			#child_params
		};
		let self_params_new = quote! {
			NEW_CHILD_INDEX,
			NewParent,
			NodeSystem,
			NodeSystemMarker,
			EdgeSystem,
			EdgeSystemMarker,
			#child_params
		};
		let self_bounds = quote! {
			const CHILD_INDEX: usize,
			Parent: IntoNodeId,
			NodeSystem: IntoNodeSystem<NodeSystemMarker>,
			NodeSystemMarker: 'static + Send + Sync,
			EdgeSystem: IntoNodeSystem<EdgeSystemMarker>,
			EdgeSystemMarker: 'static + Send + Sync,
			#child_bounds
		};


		let self_bounds_full = quote!(
			const GRAPH_ID: usize,
			const GRAPH_DEPTH: usize,
			const CHILD_INDEX: usize,
			const PARENT_DEPTH: usize,
			// Parent: IntoNodeId,
			Parent: IntoNodeId<
				GRAPH_ID = {GRAPH_ID},
				GRAPH_DEPTH = {GRAPH_DEPTH},
				// CHILD_INDEX = {CHILD_INDEX},
				PARENT_DEPTH = {PARENT_DEPTH}
				>,
			NodeSystem: IntoNodeSystem<NodeSystemMarker>,
			NodeSystemMarker: 'static + Send + Sync,
			EdgeSystem: IntoNodeSystem<EdgeSystemMarker>,
			EdgeSystemMarker: 'static + Send + Sync,
			#child_bounds
		);
		// let self_params_full = quote!(
		// 	GRAPH_ID,
		// 	GRAPH_DEPTH,
		// 	PARENT_DEPTH,
		// 	#self_params
		// );


		Self {
			ident,
			num_edges,
			self_params,
			phantom_types,
			self_bounds,
			child_params,
			child_bounds,
			self_params_new,
			self_bounds_full,
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
}
