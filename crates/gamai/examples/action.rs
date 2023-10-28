#![feature(associated_const_equality, generic_const_exprs)]
#![allow(incomplete_features)]
//this example is used for macro expansion, for usage see the `tests` directory
fn main() {}

#[derive(bevy_ecs::prelude::Component)]
struct MyBundle;

#[gamai::action(
	// props={foo=MyBundle,bar=MyBundle},
	props=(MyBundle,MyBundle),
	components=(MyBundle,MyBundle))
]
pub fn action<N: gamai::AiNode>() {}
// #[gamai::action]
// pub fn bevy_system() {}
