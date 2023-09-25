use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use rstml::node::KeyedAttribute;
use rstml::node::Node;
use rstml::node::NodeAttribute;
use rstml::node::NodeElement;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use syn::spanned::Spanned;
use syn::Result;

static NODE_ID: AtomicUsize = AtomicUsize::new(0);

pub struct NodeConfig<'a> {
	pub node: &'a NodeElement,
	pub node_system: TokenStream,
	pub edge_system: TokenStream,
	pub before_system:TokenStream,
	pub after_system:TokenStream,
	pub node_id: usize,
	pub graph_id: usize,
	pub graph_depth: usize,
	pub child_index: usize,
	pub parent_depth: usize,
	pub children: Vec<NodeConfig<'a>>,
}

impl<'a> NodeConfig<'a> {
	pub fn root(node: &'a Node, graph_id: usize) -> Result<Self> {
		let node = match node {
			Node::Element(el) => match el.open_tag.name.to_string().as_str() {
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
		node: &'a NodeElement,
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
				Node::Element(el) => Some(el),
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
				NodeAttribute::Block(block) => {
					return Err(syn::Error::new(
						block.span(),
						format!("block attributes not currently supported"),
					));
				}
				NodeAttribute::Attribute(attr) => {
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

	fn to_tokens_children(&self) -> TokenStream {
		self.children
			.iter()
			.map(|c| {
				let child = c.to_tokens_type();
				quote!(#child,)
			})
			.collect()
	}

	pub fn to_tokens_type(&self) -> TokenStream {
		let ident = self.ident();
		let children = self.to_tokens_children();
		let NodeConfig {
			node_system,
			edge_system,
			node_id,
			graph_id,
			graph_depth,
			child_index,
			parent_depth,
			..
		} = self;

		quote! {
			#ident<
			#node_system,
			#edge_system,
			#node_id,
			#graph_id,
			#graph_depth,
			#child_index,
			#parent_depth,
			#children
			>
		}
	}
	// fn to_tokens_default(&self) -> TokenStream {
	// 	let ident = self.ident();
	// 	let NodeConfig {
	// 		node_system,
	// 		edge_system,
	// 		node_id,
	// 		graph_id,
	// 		graph_depth,
	// 		child_index,
	// 		parent_depth,
	// 		..
	// 	} = self;

	// 	quote! {
	// 		#ident::<
	// 		#node_system,
	// 		#edge_system,
	// 		#node_id,
	// 		#graph_id,
	// 		#graph_depth,
	// 		#child_index,
	// 		#parent_depth,
	// 		>::default()
	// 	}
	// }
}
