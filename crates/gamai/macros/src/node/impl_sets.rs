// use super::*;
// use proc_macro2::Ident;
// use proc_macro2::Span;
// use proc_macro2::TokenStream;
// use quote::quote;


// pub fn impl_sets(node: &NodeParser) -> TokenStream {
// 	let NodeParser {
// 		ident,
// 		self_params,
// 		self_bounds,
// 		..
// 	} = node;

// 	let child_edge_ident =
// 		Ident::new(&format!("{ident}ChildEdgeSet"), Span::call_site());
// 	let node_ident = Ident::new(&format!("{ident}NodeSet"), Span::call_site());
// 	let child_node_ident =
// 		Ident::new(&format!("{ident}ChildNodeSet"), Span::call_site());

// 	quote! {
// 		#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
// 		pub struct #child_edge_ident;
// 		#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
// 		pub struct #node_ident;
// 		#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
// 		pub struct #child_node_ident;

// 		impl<#self_bounds> NodeSets for #ident<#self_params> {
// 			fn node_pre_update(&self) -> impl SystemSet { #child_edge_ident }
// 			fn node_update(&self) -> impl SystemSet { #node_ident }
// 			fn node_post_update(&self) -> impl SystemSet { #child_node_ident }
// 		}
// 	}
// }
