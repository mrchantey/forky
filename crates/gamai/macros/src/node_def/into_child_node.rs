use super::*;
use proc_macro2::TokenStream;
use quote::quote;

pub fn into_child_node(node: &NodeParser) -> TokenStream {
	let NodeParser {
		ident,
		num_children,
		// self_bounds,
		self_params,
		..
	} = node;

	let child_fields_self = child_fields_self(*num_children);

	// let node_id_params = node_id_params();
	let node_id_bounds = node_id_bounds();
	let node_id_bounds_new = node_id_bounds_new();
	let node_id_params_new = node_id_params_new();
	let child_params_new = child_params_new(*num_children);
	let child_bounds_new = child_bounds_new(*num_children);
	let child_bounds_into_new = child_bounds_into_new(*num_children);

	let bounds_old_and_new = quote! {
		#node_id_bounds,
		#node_id_bounds_new,
		Attr: IntoAttributes,
		#child_bounds_into_new
		#child_bounds_new
	};
	let self_params_new = quote! {
		#node_id_params_new,
		Attr,
		#child_params_new
	};

	quote! {
			impl<#bounds_old_and_new> IntoChildNode<#node_id_params_new> for #ident<#self_params> {
				type Out = #ident<#self_params_new>;
				fn into_child_node(self) -> #ident<#self_params_new>{
					#ident::<#self_params_new>{
						attributes: self.attributes,
						#child_fields_self
					}
				}
			}
	}
}

// returns `child0, child1, ..`
fn child_fields_self(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_name(index);
			quote!(#field: self.#field.into_child_node(),)
		})
		.collect()
}

fn child_params_new(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let child_ident_new = child_type_name_new(index);
			quote!(#child_ident_new,)
		})
		.collect()
}
fn child_bounds_new(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let child_ident_new = child_type_name_new(index);
			quote!(#child_ident_new: AiNode,)
		})
		.collect()
}
fn child_bounds_into_new(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let child_ident = child_type_name(index);
			let child_ident_new = child_type_name_new(index);
			quote! {#child_ident:AiNode + IntoChildNode<
				NEW_GRAPH_ID,
				{ NEW_GRAPH_DEPTH + 1 },
				#index,
				0,//TODO correct NodeId constraint
				{ NEW_PARENT_DEPTH + 1 },
				#child_ident_new,
			>,}
		})
		.collect()
}

// fn child_params_new(num_children: usize) -> TokenStream {
// 	(0..num_children)
// 		.map(|index| {
// 			let ty = child_type_name_new(index);
// 			quote!(#ty,)
// 		})
// 		.collect()
// }

// fn child_bounds_new(num_children: usize) -> TokenStream {
// 	(0..num_children)
// 		.map(|index| {
// 			let ty = child_type_name_new(index);
// 			quote!(#ty: AiNode,)
// 		})
// 		.collect()
// }
