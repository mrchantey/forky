use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Index;

pub fn impl_plugin(agent: &Agent) -> TokenStream {
	let Agent { ident, builder, .. } = agent;
	let AgentBuilder {
		builder_ident,
		builder_params,
		builder_bounds,
		..
	} = builder;

	let choice_systems = choice_systems(agent);
	let configure_sets = configure_sets(agent);

	quote!(
		impl<#builder_params> Plugin for #builder_ident<#builder_params>
		where #builder_bounds {
			fn build(&self, app: &mut App) {
				#configure_sets
				self.solver
					.add_agent_system::<#ident>(app, #ident.solver_set());
				#choice_systems
			}
		}
	)
}

fn configure_sets(agent: &Agent) -> TokenStream {
	let Agent { ident, .. } = agent;
	quote! {
		app.configure_set(Update,#ident.factor_set().before(#ident.solver_set()));
		app.configure_set(Update,#ident.solver_set().before(#ident.action_set()));
	}
}


fn choice_systems(agent: &Agent) -> TokenStream {
	(0..agent.num_choices)
		.map(|index| {
			let Agent { ident, .. } = agent;
			let phantom = choice_phantom(agent, index);
			let index = Index::from(index);
			quote!(self.choices.#index.add_choice_systems::<#phantom>(app, &#ident);)
		})
		.collect()
}

// fn all_factors_nested_defaults(agent: &Agent) -> TokenStream {
// 	(0..agent.num_choices)
// 		// .rev()
// 		.fold(TokenStream::new(), |prev, index| {
// 			let ident = factor_type(agent, index);
// 			quote!((#ident::default(), #prev))
// 		})
// 		.into_token_stream()
// }
