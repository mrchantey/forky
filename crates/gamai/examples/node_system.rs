//this example is used for macro expansion, for usage see the `tests` directory
#![feature(return_position_impl_trait_in_trait, associated_const_equality)]
fn main() {}

#[gamai::node_system]
pub fn my_system<N: gamai::AiNode>() {}
