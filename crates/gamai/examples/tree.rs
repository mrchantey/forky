#![feature(
	return_position_impl_trait_in_trait,
	associated_const_equality,
)]
//this example is for macro expansion, for usage see the `tests` directory
use gamai::*;
fn my_system() {}

fn main() {
	// let tree1 = tree! {<my_system/>};
	// let _ = tree1.bundle();
	// let _ = tree1.plugin();
	let _tree2 = tree! {
		<my_system>
			<my_system/>
		</my_system>
	};
}
