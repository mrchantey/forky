use super::*;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;


pub fn impl_sets(node: &AiNode) -> TokenStream {
	let AiNode { ident, .. } = node;

	let edge_ident =
		Ident::new(&format!("{ident}EdgeSystemSet"), Span::call_site());
	let node_ident = Ident::new(&format!("{ident}NodeSet"), Span::call_site());
	let action_ident =
		Ident::new(&format!("{ident}ActionSet"), Span::call_site());

	quote! {
		#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
		pub struct #edge_ident;
		#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
		pub struct #node_ident;
		#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
		pub struct #action_ident;

		impl NodeSets for #ident {
			fn edge_set(&self) -> impl SystemSet { #edge_ident }
			fn node_set(&self) -> impl SystemSet { #node_ident }
			fn action_set(&self) -> impl SystemSet { #action_ident }
		}
	}
}
