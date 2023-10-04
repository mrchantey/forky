use super::*;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

/// returns childN
pub fn child_field_name(index: usize) -> TokenStream {
	field_ident("child", index).to_token_stream()
}

/// Returns ChildN
pub fn child_type_name(index: usize) -> TokenStream {
	field_ident("Child", index).to_token_stream()
}
/// Returns ChildN
// pub fn into_child_type_name(index: usize) -> TokenStream {
// 	field_ident("IntoChild", index).to_token_stream()
// }

/// returns (Child0,(Child1,..))
pub fn _child_params_nested(node: &NodeParser) -> TokenStream {
	(0..node.num_edges)
		// .rev()
		.fold(TokenStream::new(), |prev, index| {
			let ident = child_type_name(index);
			quote!((#ident, #prev))
		})
		.into_token_stream()
}
/// returns (AiBundle<Child0>,(AiBundle<Child1>,..))
pub fn child_bundles_nested(node: &NodeParser) -> TokenStream {
	(0..node.num_edges)
		// .rev()
		.fold(TokenStream::new(), |prev, index| {
			let ident = child_type_name(index);
			quote!((AiBundle<#ident>, #prev))
		})
		.into_token_stream()
}


pub fn child_generics(num_children: usize) -> (TokenStream, TokenStream) {
	let params = (0..num_children)
		.map(|index| {
			let ty = child_type_name(index);
			quote!(#ty,)
		})
		.collect();

	let bounds = (0..num_children)
		.map(|index| {
			let ty = child_type_name(index);
			// let into_ty = into_child_type_name(index);
			quote!(
				// #into_ty: IntoNode<#index,Self>,
				// #ty:#into_ty::Out,
				#ty:AiNode,
			)
		})
		.collect();

	(params, bounds)
}

/// returns `child0: Child0, child1: Child1, ..`
pub fn child_fields_def(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_name(index);
			let ty = child_type_name(index);
			quote!(#field: #ty,)
		})
		.collect()
}
pub fn child_fields_args(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_name(index);
			let ty = child_type_name(index);
			// let ty = into_child_type_name(index);
			quote!(#field: impl IntoNode<#index,Self,Out=#ty>,)
			// quote!(#field: #ty,)
		})
		.collect()
}
// returns `child0, child1, ..`
pub fn child_fields_self(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_name(index);
			quote!(#field: self.#field,)
		})
		.collect()
}
pub fn child_fields_into_node(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let field = child_field_name(index);
			quote!(#field: #field.into_node(),)
			// quote!(#field: #field.into_node::<#index,Self>(),)
		})
		.collect()
}

// pub fn child_types(num_edges: usize) -> TokenStream {
// 	(0..num_edges)
// 		.map(|index| {
// 			let child_ident = child_type_name(index);
// 			quote!(type #child_ident: AiNode;)
// 		})
// 		.collect()
// }
// pub fn child_impl(num_edges: usize) -> TokenStream {
// 	(0..num_edges)
// 		.map(|index| {
// 			let child_ident = child_type_name(index);
// 			quote!(type #child_ident = #child_ident;)
// 		})
// 		.collect()
// }
// pub fn child_funcs_def(num_edges: usize) -> TokenStream {
// 	(0..num_edges)
// 		.map(|index| {
// 			let child_field = child_field_name(index);
// 			let child_type = child_type_name(index);
// 			quote! {
// 				fn #child_field(self)->impl FnOnce()->#child_type;
// 			}
// 		})
// 		.collect()
// }
// pub fn child_funcs_impl(num_edges: usize) -> TokenStream {
// 	(0..num_edges)
// 		.map(|index| {
// 			let child_field = child_field_name(index);
// 			let child_type = child_type_name(index);
// 			quote! {
// 				fn #child_field(self)->impl FnOnce()->#child_type{
// 					let parent = self();
// 					move || parent.#child_field
// 				}
// 			}
// 		})
// 		.collect()
// }
