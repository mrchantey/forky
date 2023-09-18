use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Index;

pub fn impl_plugin(node: &NodeParser) -> TokenStream {
	let NodeParser { ident, builder, .. } = node;
	let NodePluginParser {
		builder_ident,
		builder_params,
		builder_bounds,
		..
	} = builder;

	let child_node_systems = child_node_systems(node);
	let configure_sets = configure_sets(node);

	quote!(
		impl<#builder_params> #builder_ident<#builder_params>
		where #builder_bounds {
			fn build(&self, schedule: &mut Schedule) {
				#configure_sets
				self.node.add_node_system::<#ident>(schedule, #ident.node_set());
				#child_node_systems
			}
		}
		// deprecated use of bevy::Plugin
		// impl<#builder_params> bevy::prelude::Plugin for #builder_ident<#builder_params>
		// where #builder_bounds {
		// 	fn build(&self, app: &mut bevy::prelude::App) {
		// 		app.init_schedule(bevy::prelude::Update);
		// 		let mut schedule = app.get_schedule_mut(bevy::prelude::Update).unwrap();
		// 		#configure_sets
		// 		self.node.add_node_system::<#ident>(schedule, #ident.node_set());
		// 		#child_node_systems
		// 	}
		// }
	)
}

fn configure_sets(node: &NodeParser) -> TokenStream {
	let NodeParser { ident, .. } = node;
	quote! {
		schedule.configure_set(#ident.child_edge_set().before(#ident.node_set()));
		schedule.configure_set(#ident.node_set().before(#ident.child_node_set()));
	}
}


fn child_node_systems(node: &NodeParser) -> TokenStream {
	(0..node.num_edges)
		.map(|index| {
			let NodeParser { ident, .. } = node;
			let phantom = edge_phantom(node, index);
			let index = Index::from(index);
			quote!(self.edges.#index.add_edge_systems::<#phantom>(schedule, &#ident);)
		})
		.collect()
}

// fn all_edges_nested_defaults(node: &AiNode) -> TokenStream {
// 	(0..node.num_edges)
// 		// .rev()
// 		.fold(TokenStream::new(), |prev, index| {
// 			let ident = edge_type(node, index);
// 			quote!((#ident::default(), #prev))
// 		})
// 		.into_token_stream()
// }
