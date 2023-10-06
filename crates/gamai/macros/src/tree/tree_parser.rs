use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use rstml::node::KeyedAttribute;
use syn::spanned::Spanned;
use syn::Result;


type XmlNode = rstml::node::Node;
type XmlNodeAttribute = rstml::node::NodeAttribute;
type XmlNodeElement = rstml::node::NodeElement;

pub struct TreeParser<'a> {
	pub node: &'a XmlNodeElement,
	pub graph_id: usize,
	pub child_index: usize,
	pub child_index_of_parent: usize,
	pub graph_depth: usize,
	pub node_system: TokenStream,
	pub edge_system: TokenStream,
	pub before_system: TokenStream,
	pub after_system: TokenStream,
	pub children: Vec<TreeParser<'a>>,
}

impl<'a> TreeParser<'a> {
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
		Ok(Self::new(node, graph_id, 0, 0, 0)?)
	}

	fn new(
		node: &'a XmlNodeElement,
		graph_id: usize,
		graph_depth: usize,
		child_index: usize,
		child_index_of_parent: usize,
	) -> Result<Self> {
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
				Self::new(child, graph_id, graph_depth + 1, index, child_index)
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
			graph_id,
			graph_depth,
			child_index,
			child_index_of_parent,
		})
	}

	fn from_node_system_ident(&self) -> TokenStream {
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

	pub fn to_instance(&self) -> TokenStream {
		// let NodeConfig { graph_id, .. } = self;
		let ident = self.tree_ident();
		let tokens_root = self.to_instance_tokens(true);
		let tokens_child = self.to_instance_tokens(false);
		quote! {
			{
				#[derive(Clone,Copy)]
				struct #ident;
				impl IntoTree for #ident{
					fn get_into_root_node(self) -> impl IntoRootNode{
						#tokens_root
					}
					fn get_into_child_node<const CHILD_INDEX: usize, Parent: IntoNodeId>(self)
						-> impl IntoChildNode<CHILD_INDEX, Parent>{
						#tokens_child
					}
				}
				#ident
			}
		}
	}

	pub fn to_instance_tokens(&self, is_root: bool) -> TokenStream {
		let TreeParser {
			node,
			graph_id,
			child_index,
			node_system,
			edge_system,
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
			if is_root {
				quote! {
					move || #name.get_into_root_node()
				}
			} else {
				quote! {
					#name.get_into_child_node()
				}
			}
		} else {
			let ident = self.from_node_system_ident();
			let child_types = self.child_types();
			let child_instances = self.child_instances();
			let parent_id = if is_root {
				quote! {RootParent<#graph_id>}
			} else {
				self.parent_id()
			};
			quote! {
				#ident::<
				#child_index, #parent_id,
				_,_,_,_, //these are for NodeSystem, NodeSystemMarker, EdgeSystem, EdgeSystemMarker
				#child_types
				>::new(|| #node_system,|| #edge_system,#child_instances)
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
				let child = c.to_instance_tokens(false);
				quote!(move || {#child},)
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
	fn tree_ident(&self) -> Ident {
		Ident::new(
			format!("AutoGenTree{}", self.graph_id).as_str(),
			self.node.span(),
		)
	}
	fn parent_id(&self) -> TokenStream {
		let TreeParser {
			graph_id,
			graph_depth,
			child_index_of_parent,
			..
		} = self;
		let parent_depth = graph_depth.checked_sub(1).unwrap_or(0);
		let grandparent_depth = parent_depth.checked_sub(1).unwrap_or(0);
		quote! {
			PhantomNodeId<
			#graph_id,
			#parent_depth,
			#child_index_of_parent,
			#grandparent_depth>
		}
	}
}
