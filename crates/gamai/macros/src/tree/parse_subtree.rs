use super::attribute_parser::AttributeParser;
use crate::utils::props_name;
use proc_macro2::TokenStream;
use quote::quote;
use rstml::node::NodeElement;
use syn::Result;



pub fn parse_subtree(node: &NodeElement) -> Result<TokenStream> {
	let name = node.name();

	let prop_name = props_name(name);

	let attributes = AttributeParser::from_node(node)?;

	let props = attributes
		.other_props
		.iter()
		.map(|attr| {
			let key = &attr.key;
			let value = &attr.value();
			quote! {
				.#key(#value)
			}
		})
		.collect::<TokenStream>();


	Ok(quote! {
		{
			let props = #prop_name::builder()#props.build();
			#name(props)
		}
	})
}
