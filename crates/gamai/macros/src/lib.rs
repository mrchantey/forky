#![feature(associated_type_bounds)]
use proc_macro::TokenStream;
mod agent;
use agent::*;
mod choice_system;
use choice_system::*;
mod agent_system;
use agent_system::*;
mod utility;
use utility::*;

#[proc_macro_attribute]
pub fn agent(attr: TokenStream, item: TokenStream) -> TokenStream {
	Agent::parse(attr, item)
}

#[proc_macro_attribute]
pub fn choice_system(attr: TokenStream, item: TokenStream) -> TokenStream {
	parse_choice_system(attr, item)
}
#[proc_macro_attribute]
pub fn agent_system(attr: TokenStream, item: TokenStream) -> TokenStream {
	parse_agent_system(attr, item)
}
