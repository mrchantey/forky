use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use rstml::node::KeyedAttribute;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use syn::spanned::Spanned;
use syn::Result;


type XmlNode = rstml::node::Node;
type XmlNodeAttribute = rstml::node::NodeAttribute;
type XmlNodeElement = rstml::node::NodeElement;


static NODE_ID: AtomicUsize = AtomicUsize::new(0);

pub struct NodeConfig<'a> {
	pub node: &'a XmlNodeElement,
	pub node_id: usize,
	pub graph_id: usize,
	pub graph_depth: usize,
	pub child_index: usize,
	pub parent_depth: usize,
	pub node_system: TokenStream,
	pub edge_system: TokenStream,
	pub before_system: TokenStream,
	pub after_system: TokenStream,
	pub children: Vec<NodeConfig<'a>>,
}

impl<'a> NodeConfig<'a> {
	// pub fn num_children(&self) -> usize { self.children.len() }

	pub fn root(node: &'a XmlNode, graph_id: usize) -> Result<Self> {
		let node = match node {
			XmlNode::Element(el) => match el.open_tag.name.to_string().as_str()
			{
				"edge" => {
					todo!("handle edge parent, multiple edge children")
				}
				_ => Ok(el),
			},
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
		let node_id = NODE_ID.fetch_add(1, Ordering::SeqCst);
		let parent_depth = graph_depth.checked_sub(1).unwrap_or(0);
		let mut edge_system = quote!(gamai::empty_node);
		let mut before_system = quote!(gamai::empty_node);
		let mut after_system = quote!(gamai::empty_node);

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
						"edge" => {
							edge_system = attr
								.value()
								.map(|a| a.to_token_stream())
								.ok_or_else(|| has_no_value(attr))?;
						}
						"before" => {
							before_system = attr
								.value()
								.map(|a| a.to_token_stream())
								.ok_or_else(|| has_no_value(attr))?;
						}
						"after" => {
							after_system = attr
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

		let node_system = node.name().to_token_stream();

		Ok(Self {
			node,
			children,
			node_system,
			edge_system,
			before_system,
			after_system,
			node_id,
			graph_id,
			graph_depth,
			child_index,
			parent_depth,
		})
	}

	fn ident(&self) -> TokenStream {
		let ident = syn::Ident::new(
			&format!("Node{}", self.children.len()),
			Span::call_site(),
		);
		if self.children.len() <= 16 {
			quote!(gamai::#ident)
		} else {
			ident.to_token_stream()
		}
	}


	pub fn to_instance_tokens(&self) -> TokenStream {
		let ident = self.ident();
		let NodeConfig {
			node_id,
			graph_id,
			graph_depth,
			child_index,
			parent_depth,
			node_system,
			edge_system,
			..
		} = self;

		let child_types = self.child_types();
		let child_instances = self.child_instances();

		quote! {
			#ident::<
			#node_id,
			#graph_id,
			#graph_depth,
			#child_index,
			#parent_depth,
			_,_,_,_, //NodeSystem,NodeSystemMarker,EdgeSystem,EdgeSystemMarker
			#child_types
			>::new(|| #node_system,|| #edge_system,#child_instances)
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
}
