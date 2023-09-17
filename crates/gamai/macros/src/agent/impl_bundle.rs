use super::*;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::quote;

pub fn impl_bundle(agent: &Agent) -> TokenStream {
	let Agent { ident, vis, .. } = agent;
	let ident = Ident::new(&format!("{ident}Bundle"), ident.span());
	let choice_states = choice_states(agent);
	quote! {
		#[derive(Bundle, Default)]
		#vis struct #ident{
			#choice_states
		}
	}
}
fn choice_states(agent: &Agent) -> TokenStream {
	(0..agent.num_choices)
		.map(|index| {
			let factor_field = field_ident("factor", index);
			let factor_type = factor_type(agent, index);
			// let action_field = field_ident("action", index);
			// let action_type = action_type(agent, index);
			quote!(
				#factor_field: #factor_type,
				// #action_field: #action_type,
			)
		})
		.collect()
}
