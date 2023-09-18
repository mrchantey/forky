use super::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn edge_type(agent: &Agent, index: usize) -> TokenStream {
	let phantom = choice_phantom(agent, index);
	quote!(ChoiceEdgeState<#phantom>)
}
// pub fn action_type(agent: &Agent, index: usize) -> TokenStream {
// 	let phantom = choice_phantom(agent, index);
// 	quote!(ChoiceActionState<#phantom>)
// }
pub fn action_default(agent: &Agent, index: usize) -> TokenStream {
	let phantom = choice_phantom(agent, index);
	quote!(ChoiceActionState::<#phantom>::default())
}

pub fn choice_phantom(agent: &Agent, index: usize) -> TokenStream {
	let ident = &agent.ident;
	quote!(ChoicePhantom<#ident,#index>)
}
