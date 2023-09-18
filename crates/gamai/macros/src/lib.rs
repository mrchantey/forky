#![feature(associated_type_bounds)]
use proc_macro::TokenStream;
mod node;
use node::*;
mod edge_system;
use edge_system::*;
mod node_system;
use node_system::*;
mod utility;
use utility::*;

#[proc_macro_attribute]
pub fn node(attr: TokenStream, item: TokenStream) -> TokenStream {
	NodeParser::parse(attr, item)
}

#[proc_macro_attribute]
pub fn edge_system(attr: TokenStream, item: TokenStream) -> TokenStream {
	parse_edge_system(attr, item)
}
#[proc_macro_attribute]
pub fn node_system(attr: TokenStream, item: TokenStream) -> TokenStream {
	parse_node_system(attr, item)
}
