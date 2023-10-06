#![feature(return_position_impl_trait_in_trait, associated_const_equality)]
//this example is for macro expansion, for usage see the `tests` directory
use gamai::*;
fn my_system() {}

fn main() {
	let _tree0 = tree! {<MyTree/>};
	let _tree1 = tree! {
		<my_system>
			<my_system>
				<my_system>
					<my_system/>
					<my_system/>
					<MyTree/>
				</my_system>
			</my_system>
		</my_system>
	};
}


#[tree_builder]
pub fn MyTree() -> impl IntoTree {
	tree! {
		<my_system>
			<MyTree/>
		</my_system>
	}
}
