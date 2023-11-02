// use super::attribute_parser::AttributeParser;
use super::*;
use crate::utils::parent_element;
// use crate::utils::props_name;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
// use rstml::node::NodeElement;
use syn::Result;



pub fn parse_action_node(tree: &TreeParser) -> Result<TokenStream> {
	let TreeParser {
		node,
		graph_id,
		children,
		..
	} = tree;

	let num_children = children.len();
	let ident = from_action_ident(num_children);
	let child_types = child_types(num_children);
	let child_instances = child_instances(children)?;
	let attribute = AttributeParser::from_node(node)?;
	let actions = &attribute.actions;
	let actions = if let Some(deff) = &attribute.apply_deferred {
		quote!(#actions.apply_deferred(#deff))
	} else {
		actions.clone()
		// //TODO shouldnt need to do this, some issue with ActionConfig
		// quote!(#actions.into_action_config())
		// // quote!(#action)
	};
	let props = attribute.to_prop_bundle();
	Ok(quote! {
		#ident::<gamai::node::TreePathRoot<#graph_id>,
		_,_, //for action & props
		#child_types
		>::new(#actions, #props, #child_instances)
	})
}

fn from_action_ident(num_children: usize) -> TokenStream {
	let ident = parent_element(num_children);
	if num_children <= 16 {
		quote!(gamai::node::#ident)
	} else {
		ident.to_token_stream()
	}
}

fn child_types(num_children: usize) -> TokenStream {
	std::iter::repeat(quote! {_,}).take(num_children).collect()
}

fn child_instances(children: &Vec<TreeParser>) -> Result<TokenStream> {
	Ok(children
		.iter()
		.map(|c| {
			let child = c.to_tokens()?;
			Ok(quote!(#child,))
		})
		.collect::<Result<Vec<_>>>()?
		.into_iter()
		.collect::<TokenStream>())
}
