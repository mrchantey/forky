use proc_macro2::TokenStream;
use quote::quote;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;


// ALSO UPDATE tree_parser.rs:199
pub fn node_id_bounds() -> TokenStream {
	quote! {
		const GRAPH_ID: usize,
		const GRAPH_DEPTH: usize,
		const CHILD_INDEX: usize,
		const NODE_ID: usize,
		const : usize
	}
}

pub fn node_id_params() -> TokenStream {
	quote! {
		GRAPH_ID,
		GRAPH_DEPTH,
		CHILD_INDEX,
		NODE_ID,
		PARENT_DEPTH
	}
}

static NODE_ID: AtomicUsize = AtomicUsize::new(0);

pub fn node_id_params_child(child_index: usize) -> TokenStream {
	let node_id = NODE_ID.fetch_add(1, Ordering::SeqCst);
	quote! {
		GRAPH_ID,
		GRAPH_DEPTH,
		// {GRAPH_DEPTH+1},
		#child_index,
		#node_id,
		PARENT_DEPTH
		// {PARENT_DEPTH+1}
	}
}

pub fn node_id_bounds_new() -> TokenStream {
	quote! {
		const NEW_GRAPH_ID: usize,
		const NEW_GRAPH_DEPTH: usize,
		const NEW_CHILD_INDEX: usize,
		const NEW_NODE_ID: usize,
		const NEW_PARENT_DEPTH: usize
	}
}

pub fn node_id_params_new() -> TokenStream {
	quote! {
		NEW_GRAPH_ID,
		NEW_GRAPH_DEPTH,
		// {GRAPH_DEPTH+1},
		NEW_CHILD_INDEX,
		NEW_NODE_ID,
		// {PARENT_DEPTH+1}
		NEW_PARENT_DEPTH
	}
}
