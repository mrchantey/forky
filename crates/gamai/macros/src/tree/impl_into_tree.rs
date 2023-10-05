// use crate::*;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

static GRAPH_ID: AtomicUsize = AtomicUsize::new(0);


pub fn impl_into_tree(ident: &Ident) -> TokenStream {
	// let ident = &func.sig.ident;
	// let inner_ident = func_inner_ident(&func.sig.ident);
	let graph_id = GRAPH_ID.fetch_add(1, Ordering::SeqCst);

	quote! {
		//TODO impl IntoNode

		impl IntoRootNode for #ident {
			fn into_root_node(&self) -> impl AiNode {
				self.into_child_node::<0, #graph_id, 0, 0, RootParent<#graph_id>>()
			}
		}

		impl IntoChildNode for #ident{
			fn into_child_node<
				const CHILD_INDEX: usize,
				const GRAPH_ID: usize,
				const PARENT_DEPTH: usize,
				const GRANDPARENT_DEPTH: usize,
				Parent,
			>(self) -> impl AiNode
			where
				Parent: IntoNodeId<
					GRAPH_ID = { GRAPH_ID },
					GRAPH_DEPTH = { PARENT_DEPTH },
					PARENT_DEPTH = { GRANDPARENT_DEPTH },
				>,
			{
				//temp
				Node0::<CHILD_INDEX, Parent, _, _, _, _>::new(|| || {}, || || {})
				// #inner_ident::<
				// 	CHILD_INDEX,
				// 	GRAPH_ID,
				// 	PARENT_DEPTH,
				// 	GRANDPARENT_DEPTH,
				// 	Parent,
				// >()
			}
		}
	}
}
