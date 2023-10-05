#![feature(
	return_position_impl_trait_in_trait,
	associated_const_equality,
	// generic_const_exprs
)]
//this example is for macro expansion, for usage see the `tests` directory
use gamai::*;
fn my_system() {}

fn main() {
	// let child = tree! {
	// 	<_my_system/>
	// };
	// let child = child();
	// child().into_no
	// let parent = Node1::<0, RootParent<0>, _, _, _, _, _>::new(
	// 	|| _my_system,
	// 	|| _my_system,
	// 	child,
	// );
	let my_tree = tree! {
		<my_system>
			// <child/>
		</my_system>
	};
	let _ = AiBundle::new(my_tree);
	let _ = AiBundle::new(my_tree);
	let _ = AiPlugin::new(my_tree);
	let _ = AiPlugin::new(my_tree);
}
