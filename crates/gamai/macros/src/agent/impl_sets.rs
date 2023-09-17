use super::*;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;


pub fn impl_sets(agent: &Agent) -> TokenStream {
	let Agent { ident, .. } = agent;

	let factor_ident =
		Ident::new(&format!("{ident}FactorSet"), Span::call_site());
	let solver_ident =
		Ident::new(&format!("{ident}SolverSet"), Span::call_site());
	let action_ident =
		Ident::new(&format!("{ident}ActionSet"), Span::call_site());

	quote! {
		#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
		pub struct #factor_ident;
		#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
		pub struct #solver_ident;
		#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
		pub struct #action_ident;

		impl SolverSets for #ident {
			fn factor_set(&self) -> impl SystemSet { #factor_ident }
			fn solver_set(&self) -> impl SystemSet { #solver_ident }
			fn action_set(&self) -> impl SystemSet { #action_ident }
		}
	}
}

