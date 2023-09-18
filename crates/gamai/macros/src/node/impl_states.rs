use super::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn edge_type(node: &AiNode, index: usize) -> TokenStream {
	let phantom = choice_phantom(node, index);
	quote!(ChoiceEdgeState<#phantom>)
}
// pub fn action_type(node: &AiNode, index: usize) -> TokenStream {
// 	let phantom = choice_phantom(node, index);
// 	quote!(ChoiceActionState<#phantom>)
// }
pub fn action_default(node: &AiNode, index: usize) -> TokenStream {
	let phantom = choice_phantom(node, index);
	quote!(ChoiceActionState::<#phantom>::default())
}

pub fn choice_phantom(node: &AiNode, index: usize) -> TokenStream {
	let ident = &node.ident;
	quote!(ChoicePhantom<#ident,#index>)
}
