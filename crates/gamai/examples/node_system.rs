#![feature(associated_type_bounds, return_position_impl_trait_in_trait)]
use bevy_ecs::prelude::*;
use gamai::*;

//this example is used for macro expansion, for usage see the `tests` directory
fn main() {}

#[node_system]
pub fn my_node<N: AiNode>() {}
