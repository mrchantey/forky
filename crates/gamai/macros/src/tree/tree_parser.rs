use super::*;
use proc_macro2::TokenStream;
use rstml::node::NodeName;
use syn::spanned::Spanned;
use syn::Result;

pub type XmlNode = rstml::node::Node;
pub type XmlNodeAttribute = rstml::node::NodeAttribute;
pub type XmlNodeElement = rstml::node::NodeElement;

pub struct TreeParser<'a> {
	pub graph_id: usize,
	pub node: &'a XmlNodeElement,
	pub children: Vec<TreeParser<'a>>,
}

impl<'a> TreeParser<'a> {
	pub fn new(node: &'a XmlNode, graph_id: usize) -> Result<Self> {
		let node = match node {
			XmlNode::Element(el) => Ok(el),
			val => Err(syn::Error::new(val.span(), "Expected element node")),
		}?;

		// depth-first traversal of xml graph
		let children = node
			.children
			.iter()
			.map(|child| Self::new(child, graph_id))
			.collect::<Result<Vec<_>>>()?;

		Ok(Self {
			node,
			children,
			graph_id,
		})
	}

	pub fn to_tokens(&self) -> Result<TokenStream> {
		if is_tree(self.node.name()) {
			parse_subtree_node(self)
		} else {
			parse_action_node(self)
		}
	}
}

fn is_tree(name: &NodeName) -> bool {
	let name = name.to_string();
	if let Some(first_char) = name.chars().next() {
		if first_char.is_uppercase() {
			return true;
		}
	}
	false
}
