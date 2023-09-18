#![feature(associated_type_bounds, return_position_impl_trait_in_trait)]
use bevy::prelude::*;
use gamai::*;

//this example is used for macro expansion, for usage see the `tests` directory
fn main() {}

#[edge_system]
pub fn noop_node<C: AiEdge>(mut _query: Query<&mut ChildNodeState<C>>) {}
