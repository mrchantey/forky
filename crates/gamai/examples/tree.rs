#![feature(return_position_impl_trait_in_trait, associated_const_equality)]
//this example is for macro expansion, for usage see the `tests` directory
use gamai::*;
fn my_system() {}

fn main() {
	let my_tree = tree! {
		<my_system>
			<my_system/>
		</my_system>
	};
	let _ = AiBundle::new(my_tree);
	let _ = AiPlugin::new(my_tree);
	let _ = AiPlugin::new(my_tree);
}

tree! {MyTree,
	<my_system/>
}
