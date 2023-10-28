use crate::utils::parent_element;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use rstml::node::KeyedAttribute;
use rstml::node::NodeElement;
use syn::spanned::Spanned;
use syn::Result;

type XmlNode = rstml::node::Node;
type XmlNodeAttribute = rstml::node::NodeAttribute;
type XmlNodeElement = rstml::node::NodeElement;

pub struct TreeParser<'a> {
	pub node: &'a XmlNodeElement,
	pub graph_id: usize,
	pub child_index: usize,
	pub graph_depth: usize,
	pub attribute: AttributeParser,
	pub children: Vec<TreeParser<'a>>,
}

pub struct AttributeParser {
	// pub props:TokenStream,
	pub pre_parent_update: TokenStream,
	pub pre_update: TokenStream,
	pub update_apply_deferred: bool,
	pub update: TokenStream,
	pub post_update: TokenStream,
}
impl AttributeParser {
	pub fn from_attributes(node: &NodeElement) -> Result<Self> {
		let mut attributes = Self::default();
		attributes.update = node.name().to_token_stream();


		let has_no_value = |attr: &KeyedAttribute| {
			syn::Error::new(attr.key.span(), "this attribute must have a value")
		};
		for attribute in node.attributes() {
			match attribute {
				XmlNodeAttribute::Block(block) => {
					return Err(syn::Error::new(
						block.span(),
						format!("block attributes not currently supported"),
					));
				}
				XmlNodeAttribute::Attribute(attr) => {
					match attr.key.to_string().as_str() {
						"apply_deferred" => {
							attributes.update_apply_deferred = true
						}
						"before_parent" => {
							attributes.pre_parent_update = attr
								.value()
								.map(|a| a.to_token_stream())
								.ok_or_else(|| has_no_value(attr))?;
						}
						"before" => {
							attributes.pre_update = attr
								.value()
								.map(|a| a.to_token_stream())
								.ok_or_else(|| has_no_value(attr))?;
						}
						"after" => {
							attributes.post_update = attr
								.value()
								.map(|a| a.to_token_stream())
								.ok_or_else(|| has_no_value(attr))?;
						}
						_ => {
							return Err(syn::Error::new(
								attr.key.span(),
								format!(
									"attribute '{}' not supported",
									attr.key
								),
							));
						}
					}
				}
			}
		}


		Ok(attributes)
	}
	pub fn to_attributes_tokens(&self) -> TokenStream {
		let Self {
			update,
			pre_update,
			update_apply_deferred,
			pre_parent_update,
			post_update,
		} = self;
		quote! {
		gamai::node::Attributes::new(
			#pre_parent_update,
			#pre_update,
			#update,
			#update_apply_deferred,
			#post_update)
		}
	}
}
impl Default for AttributeParser {
	fn default() -> Self {
		Self {
			pre_parent_update: quote!(gamai::common_actions::empty_node),
			pre_update: quote!(gamai::common_actions::empty_node),
			update_apply_deferred: false,
			update: quote!(gamai::common_actions::empty_node),
			post_update: quote!(gamai::common_actions::empty_node),
		}
	}
}

impl<'a> TreeParser<'a> {
	pub fn root(node: &'a XmlNode, graph_id: usize) -> Result<Self> {
		let node = match node {
			XmlNode::Element(el) => Ok(el),
			val => Err(syn::Error::new(val.span(), "Expected element node")),
		}?;
		Ok(Self::new(node, graph_id, 0, 0)?)
	}

	fn new(
		node: &'a XmlNodeElement,
		graph_id: usize,
		graph_depth: usize,
		child_index: usize,
	) -> Result<Self> {
		let attribute = AttributeParser::from_attributes(node)?;

		let children = node
			.children
			.iter()
			.filter_map(|c| match c {
				XmlNode::Element(el) => Some(el),
				_ => None,
			})
			.enumerate()
			.map(|(index, child)| {
				Self::new(child, graph_id, graph_depth + 1, index)
			})
			// .to_owned()
			.collect::<Result<Vec<_>>>()?;

		Ok(Self {
			node,
			children,
			attribute,
			graph_id,
			graph_depth,
			child_index,
		})
	}

	fn from_action_ident(&self) -> TokenStream {
		let ident = parent_element(self.children.len());
		if self.children.len() <= 16 {
			quote!(gamai::node::#ident)
		} else {
			ident.to_token_stream()
		}
	}

	pub fn to_instance(&self) -> TokenStream {
		let tokens_root = self.to_instance_tokens();
		quote! {
			#tokens_root.into_root()
		}
	}

	pub fn to_instance_tokens(&self) -> TokenStream {
		let TreeParser {
			node,
			graph_id,
			// child_index,			// dont need to set child index because if child, intochildnode will set it
			attribute,
			..
		} = self;

		if self.is_tree() {
			if let Some(child) = self.children.first() {
				return syn::Error::new(
					child.node.span(),
					"Subtrees cannot contain additional children",
				)
				.to_compile_error()
				.into();
			}
			let name = node.name();
			// if its a tree function, call it
			quote!(#name())
		} else {
			let ident = self.from_action_ident();
			let child_types = self.child_types();
			let child_instances = self.child_instances();
			let attribute = attribute.to_attributes_tokens();
			quote! {
				#ident::<gamai::node::TreePathRoot<#graph_id>,
				_,_, //for action & props
				#child_types
				>::new(#attribute, #attribute, #child_instances)
			}
		}
	}
	fn child_types(&self) -> TokenStream {
		std::iter::repeat(quote! {_,})
			.take(self.children.len())
			.collect()
	}
	fn child_instances(&self) -> TokenStream {
		self.children
			.iter()
			.map(|c| {
				let child = c.to_instance_tokens();
				quote!(#child,)
			})
			.collect()
	}
	fn is_tree(&self) -> bool {
		let name = self.node.name().to_string();
		if let Some(first_char) = name.chars().next() {
			if first_char.is_uppercase() {
				return true;
			}
		}
		false
	}
}
