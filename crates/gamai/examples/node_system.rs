#![feature(
	associated_const_equality,
	generic_const_exprs
)]
#![allow(incomplete_features)]
//this example is used for macro expansion, for usage see the `tests` directory
fn main() {}

#[gamai::node_system]
pub fn node_system<N: gamai::AiNode>() {}
#[gamai::node_system]
pub fn bevy_system() {}
