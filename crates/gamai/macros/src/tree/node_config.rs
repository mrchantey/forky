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

pub struct NodeConfig<'a> {
	pub node: &'a XmlNodeElement,
	pub graph_id: usize,
	pub child_index: usize,
	pub child_index_of_parent: usize,
	pub graph_depth: usize,
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

	pub fn to_struct(&self, ident: Ident) -> TokenStream {
		let tokens = self.to_instance_tokens();
		let as_root = tree_as_root_ident(&ident);
		let as_child = tree_as_child_ident(&ident);
		quote! {
			// pub struct #ident;
			#[allow(non_snake_case)]
			fn #as_root()->impl IntoRootNode{
				#tokens
			}
			#[allow(non_snake_case)]
			fn #as_child<const CHILD_INDEX:usize, Parent: IntoNodeId>()->impl IntoChildNode<CHILD_INDEX,Parent>{
				#tokens
			}
		}
	}

	pub fn to_instance(&self) -> TokenStream {
		let tokens = self.to_instance_tokens();
		quote! {
			|| #tokens
		}
	}

	pub fn to_instance_tokens(&self) -> TokenStream {
		let ident = self.ident();
		let NodeConfig {
			node,
			graph_id,
			graph_depth,
			child_index,
			node_system,
			edge_system,
			child_index_of_parent,
			..
		} = self;

		// let parent_depth:usize = 0;
		// let grandparent_depth:usize = 0;
		let child_types = self.child_types();
		let child_instances = self.child_instances();
		let name = node.name().to_string();
		if is_uppercase(&name) {
			let ident = tree_as_child_ident(&Ident::new(&name, node.span()));

			let parent_id = parent_id(
				*graph_id,
				*child_index_of_parent,
				graph_depth.checked_sub(1).unwrap_or(0),
			); //not sure why we need to sub 1 here

			if let Some(child) = self.children.first() {
				return syn::Error::new(
					child.node.span(),
					"Subtrees cannot contain additional children",
				)
				.to_compile_error()
				.into();
			}
			//TODO parent child index?
			quote! {
				#ident::<#child_index,#parent_id>()
			}
		} else {
			let parent_id =
				parent_id(*graph_id, *child_index_of_parent, *graph_depth);

			quote! {
				#ident::<
				0, #parent_id,
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
				let child = c.to_instance_tokens();
				quote!(move || {#child},)
			})
			.collect()
	}
}

fn is_uppercase(val: &str) -> bool {
	if let Some(first_char) = val.chars().next() {
		if first_char.is_uppercase() {
			return true;
		}
	}
	false
}


fn tree_as_root_ident(ident: &Ident) -> Ident {
	Ident::new(&format!("{}_as_root", ident), ident.span())
}
fn tree_as_child_ident(ident: &Ident) -> Ident {
	Ident::new(&format!("{}_as_child", ident), ident.span())
}


fn parent_id(
	graph_id: usize,
	child_index_of_parent: usize,
	depth: usize,
) -> TokenStream {
	let parent_depth = depth.checked_sub(1).unwrap_or(0);
	let grandparent_depth = parent_depth.checked_sub(1).unwrap_or(0);
	quote! {
		PhantomNodeId<
		#graph_id,
		#parent_depth,
		#child_index_of_parent,
		#grandparent_depth>
	}
}
