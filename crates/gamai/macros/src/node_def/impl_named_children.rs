// use super::*;
// // use proc_macro2::Ident;
// use proc_macro2::TokenStream;
// use quote::quote;

// pub fn impl_named_children(_node: &NodeParser) -> TokenStream {
// 	// let NodeParser {
// 	// 	ident,
// 	// 	self_params,
// 	// 	self_bounds,
// 	// 	num_edges,
// 	// 	..
// 	// } = node;
// 	// let trait_ident =
// 	// 	Ident::new(&format!("NamedChildren{num_edges}"), node.ident.span());

// 	// let child_types = child_types(*num_edges);
// 	// let child_impl = child_impl(*num_edges);

// 	// let child_funcs_def = child_funcs_def(*num_edges);
// 	// let child_funcs_impl = child_funcs_impl(*num_edges);


// 	quote! {

// 		// pub trait #trait_ident<#self_bounds>{
// 		// 	#child_funcs_def
// 		// }

// 		// // impl<F,#self_bounds> #trait_ident<#self_params> for F
// 		// // 	where F:Fn()->#ident<#self_params>
// 		// impl<#self_bounds> #trait_ident<#self_params> for fn()->#ident<#self_params>
// 		// 	{
// 		// 	#child_funcs_impl
// 		// }
// 	}
// }
