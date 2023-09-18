#![feature(associated_type_bounds)]
use proc_macro::TokenStream;
mod node;
use node::*;
mod choice_system;
use choice_system::*;
mod node_system;
use node_system::*;
mod utility;
use utility::*;

#[proc_macro_attribute]
pub fn node(attr: TokenStream, item: TokenStream) -> TokenStream {
	AiNode::parse(attr, item)
}

#[proc_macro_attribute]
pub fn choice_system(attr: TokenStream, item: TokenStream) -> TokenStream {
	parse_choice_system(attr, item)
}
#[proc_macro_attribute]
pub fn node_system(attr: TokenStream, item: TokenStream) -> TokenStream {
	parse_node_system(attr, item)
}
