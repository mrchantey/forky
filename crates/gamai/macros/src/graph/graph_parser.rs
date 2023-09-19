use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use rstml::node::Node;
use rstml::node::NodeAttribute;
use rstml::node::NodeElement;
use rstml::Parser;
use rstml::ParserConfig;
// use syn::parse_macro_input;
// use syn::AttributeArgs;
// use syn::ItemFn;
// use syn::Visibility;
// use rstml::node::Node;
// use rstml::node::NodeAttribute;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use syn::spanned::Spanned;
use syn::Result;


static CNT: AtomicUsize = AtomicUsize::new(0);


pub struct GraphParser {
	pub node: TokenStream,
	pub children: Vec<GraphParser>,
	pub edge: TokenStream,
	// pub errors: TokenStream,
}

impl GraphParser {
	pub fn parse(tokens: proc_macro::TokenStream) -> Result<TokenStream> {
		let config = ParserConfig::new()
			.recover_block(true)
			.number_of_top_level_nodes(1);

		let parser = Parser::new(config);
		let (nodes, errors) = parser.parse_recoverable(tokens).split_vec();

		if let Some(node) = nodes.first() {
			let graph = Self::new(node)?;
			let n = graph.node;

			let errors = errors.into_iter().map(|e| e.emit_as_expr_tokens());
			let id = CNT.fetch_add(1, Ordering::SeqCst);
			Ok(quote! {
				#(#errors;)*
				gamai::AnonNode::<#n, #id>::default()
				// impl AnonNode<#n, #id>
				// #n
			})
		} else {
			Err(syn::Error::new(Span::call_site(), "Expected a root node"))
		}
	}

	pub fn new(node: &Node) -> Result<Self> {
		match node {
			Node::Element(el) => match el.open_tag.name.to_string().as_str() {
				"edge" => {
					todo!("handle edge parent, multiple edge children")
				}
				_ => Self::new_from_element(el),
			},
			val => Err(syn::Error::new(val.span(), "Expected element node")),
		}
	}

	pub fn new_from_element(el: &NodeElement) -> Result<Self> {
		let mut edge = quote!("default edge here");

		for attribute in el.attributes() {
			match attribute {
				NodeAttribute::Block(_block) => {
					todo!("whats a block attribute?")
				}
				NodeAttribute::Attribute(attr) => {
					match attr.key.to_string().as_str() {
						"edge" => {
							edge = attr
								.value()
								.map(|a| a.to_token_stream())
								.ok_or_else(|| {
									syn::Error::new(
										attr.key.span(),
										"edge attribute must have a value",
									)
								})?;
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

		let node = el.name().to_token_stream();

		Ok(Self {
			node,
			children: Vec::new(),
			edge,
		})
	}
}
