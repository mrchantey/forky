use super::*;
use crate::utils::*;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use syn::LitInt;

pub struct NodeParser {
	pub num_children: usize,
	/// The name of this node: `Node0`
	pub ident: Ident,
	/// all generic params for this type: `NODE_ID, GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, Child1, Child2`
	pub self_params: TokenStream,
	/// all generic bounds for this type: `const NODE_ID: usize, Child1: AiNode, ...`
	pub self_bounds: TokenStream,
	/// types of the children: `Child1, Child2`
	pub child_params: TokenStream,
	/// bound types of the children: `Child1: AiNode, Child2: AiNode`
	pub child_bounds: TokenStream,
}


impl NodeParser {
	pub fn new(num_children: usize) -> Self {
		let ident = parent_node(num_children);

		let child_params = child_params(num_children);
		let child_bounds = child_bounds(num_children);

		let self_params = quote! {
			Path,
			#child_params
		};
		let self_bounds = quote! {
			Path: TreePath,
			#child_bounds
		};

		Self {
			ident,
			num_children,
			self_params,
			self_bounds,
			child_params,
			child_bounds,
		}
	}
}

pub fn get_num_children(attr: proc_macro::TokenStream) -> syn::Result<usize> {
	if let Ok(lit) = syn::parse::<LitInt>(attr.into()) {
		let val = lit.base10_parse::<usize>()?;
		if val <= 117 {
			Ok(val)
		} else {
			Err(syn::Error::new(
				lit.span(),
				"maximum number of children is 117",
			))
		}
	} else {
		Err(syn::Error::new(
			Span::call_site(),
			"please specify number of children",
		))
	}
}




fn child_params(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let ty = child_type_name(index);
			quote!(#ty,)
		})
		.collect()
}
fn child_bounds(num_children: usize) -> TokenStream {
	(0..num_children)
		.map(|index| {
			let ty = child_type_name(index);
			quote!(#ty:AiNode,)
		})
		.collect()
}
