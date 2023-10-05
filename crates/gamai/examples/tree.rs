#![feature(
	return_position_impl_trait_in_trait,
	associated_const_equality,
	// generic_const_exprs
)]
//this example is for macro expansion, for usage see the `tests` directory
use gamai::*;
fn my_system() {}

fn main() {
	//
	// let _ = AiPlugin::new(my_tree);
	let my_tree = tree! {<my_system/>};
}



// fn my_tree() -> impl IntoTree {
// }
