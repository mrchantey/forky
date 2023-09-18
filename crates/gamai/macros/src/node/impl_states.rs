use super::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn edge_type(node: &AiNode, index: usize) -> TokenStream {
	let phantom = choice_phantom(node, index);
	quote!(ChoiceEdgeState<#phantom>)
}
// pub fn child_node_type(node: &AiNode, index: usize) -> TokenStream {
// 	let phantom = choice_phantom(node, index);
// 	quote!(ChildNodeState<#phantom>)
// }
pub fn default_child_node_state(node: &AiNode, index: usize) -> TokenStream {
	let phantom = choice_phantom(node, index);
	quote!(ChildNodeState::<#phantom>::default())
}

pub fn choice_phantom(node: &AiNode, index: usize) -> TokenStream {
	let ident = &node.ident;
	quote!(ChoicePhantom<#ident,#index>)
}
