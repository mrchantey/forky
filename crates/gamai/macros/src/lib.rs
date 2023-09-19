#![feature(associated_type_bounds)]
use proc_macro::TokenStream;
mod graph;
use graph::*;
mod node;
use node::*;
mod edge;
use edge::*;
mod utility;
use utility::*;

#[proc_macro]
pub fn graph(item: TokenStream) -> TokenStream {
	GraphParser::parse(item)
		.unwrap_or_else(syn::Error::into_compile_error)
		.into()
}
#[proc_macro_attribute]
pub fn node(attr: TokenStream, item: TokenStream) -> TokenStream {
	NodeParser::parse_node_full(attr, item)
}
#[proc_macro_attribute]
pub fn node_system(attr: TokenStream, item: TokenStream) -> TokenStream {
	NodeParser::parse_node_system(attr, item)
}

#[proc_macro_attribute]
pub fn edge(attr: TokenStream, item: TokenStream) -> TokenStream {
	parse_edge_system(attr, item)
}

#[proc_macro]
pub fn html(tokens: TokenStream) -> TokenStream { html_inner(tokens, false) }
