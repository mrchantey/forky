use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;
use syn::ItemStruct;

pub struct AgentBuilder {
	pub builder_ident: Ident,
	pub builder_params: TokenStream,
	pub builder_bounds: TokenStream,
}

impl AgentBuilder {
	pub fn new(item: &ItemStruct, num_choices: usize) -> Self {
		let ident =
			Ident::new(&format!("{}Plugin", item.ident), item.ident.span());

		let (builder_params, generic_bounds) = builder_params(num_choices);

		Self {
			builder_ident: ident,
			builder_params: builder_params,
			builder_bounds: generic_bounds,
		}
	}
}

fn builder_params(num_params: usize) -> (TokenStream, TokenStream) {
	let (choice_params, choice_bounds) = choice_generics(num_params);
	let params = quote!(Solver, #choice_params);
	let bounds = quote!(Solver: AddAgentSystem, #choice_bounds);
	(params, bounds)
}


pub fn impl_builder(agent: &Agent) -> TokenStream {
	let plugin_impl = impl_plugin(agent);

	let Agent {
		choice_params,
		// vis,
		builder,
		..
	} = agent;
	let AgentBuilder {
		builder_ident,
		builder_params,
		builder_bounds,
		..
	} = builder;

	quote! {
		// #[derive(Debug)]
		pub struct #builder_ident<#builder_params> where #builder_bounds{
			solver: Solver,
			choices: (#choice_params),
		}
		impl<#builder_params> #builder_ident<#builder_params> where #builder_bounds{
			pub fn new(solver: fn()->Solver, choices: (#choice_params))->Self
				where #builder_bounds {
				#builder_ident{ solver:solver(), choices }
			}
	}
		#plugin_impl
	}
}
