use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

pub fn impl_agent(agent: &Agent) -> TokenStream {
	let Agent { ident, .. } = agent;
	// let AgentBuilder {
	// 	builder_ident: ident,
	// 	builder_bounds,
	// 	builder_params,
	// 	..
	// } = builder;
	let world_query = all_factors_nested(agent);
	let params = agent_params_nested(agent);
	let params_deref = agent_params_deref(agent);
	let set_action = agent_set_action(agent);

	quote!(
		impl Agent for #ident
		{
			type Items = (Entity, #world_query);
			fn factors(query: &Query<Self::Items>) -> Vec<(Entity, Vec<FactorState>)> {
				query
					.iter()
					.map(|(entity, #params)| (entity, vec![#params_deref]))
					.collect()
			}
			fn set_action(commands: &mut Commands, entity: Entity, index: usize) {
				match index {
					#set_action
					_ => panic!("index out of range"),
				};
			}
		}
	)
}

fn all_factors_nested(agent: &Agent) -> TokenStream {
	(0..agent.num_choices)
		// .rev()
		.fold(TokenStream::new(), |prev, index| {
			let ident = factor_type(agent, index);
			quote!((&'static #ident, #prev))
		})
		.into_token_stream()
}
fn agent_params_nested(agent: &Agent) -> TokenStream {
	(0..agent.num_choices)
		// .rev()
		.fold(TokenStream::new(), |prev, index| {
			let ident = field_ident("factor", index);
			quote!((#ident, #prev))
		})
		.into_token_stream()
}

fn agent_params_deref(agent: &Agent) -> TokenStream {
	(0..agent.num_choices)
		.map(|index| {
			let ident = field_ident("factor", index);
			quote!(**#ident,)
		})
		.collect()
}

fn agent_set_action(agent: &Agent) -> TokenStream {
	// let Agent { ident, .. } = agent;
	(0..agent.num_choices)
		.map(|index| {
			let val = action_default(agent, index);
			quote!(#index => commands.entity(entity).insert(#val),)
		})
		.collect()
}
