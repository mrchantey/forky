#![feature(associated_type_bounds, return_position_impl_trait_in_trait)]
use bevy_ecs::prelude::*;
//this example is used for macro expansion, for usage see the `tests` directory
use gamai::*;


fn my_system() {}

fn main() {}
fn foo() -> impl AiNode {
	gamai::tree! {<my_system/>}
}
