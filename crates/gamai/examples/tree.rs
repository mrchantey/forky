#![feature(associated_type_bounds, return_position_impl_trait_in_trait)]
//this example is used for macro expansion, for usage see the `tests` directory
use gamai::*;

fn main() {
	let _ = AiPlugin::new(my_node);
	let _ = AiBundle::new(my_node);
}

fn my_system() {}
fn my_node() -> impl AiNode {
	gamai::tree! {<my_system/>}
}