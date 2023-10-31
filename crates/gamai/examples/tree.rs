#![feature(associated_const_equality, generic_const_exprs)]
#![allow(incomplete_features)]
//this example is for macro expansion, for usage see the `tests` directory
use gamai::*;

#[action(apply_deferred)]
fn my_system<N: AiNode>() {}

fn main() {
	let tree1 = || tree! {<my_system apply_deferred/>};
	// let _tree2 = || tree! {<group apply_deferred/>};
	let _bundle = TreeBundle::new(tree1);
	let _plugin = TreePlugin::new(tree1);
}
