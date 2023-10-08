//this example is used for macro expansion, for usage see the `tests` directory
#![feature(
	return_position_impl_trait_in_trait,
	associated_const_equality,
	generic_const_exprs
)]
#![allow(incomplete_features)]
use gamai::*;

fn _my_system() {}

fn main() {
	// let a = Baz::<0>::next();
	// assert_eq!(a.val(), 1);
	let tree = || {
		let child = || {
			Node0::<0, 0, 0, 0, 0, _, _, _, _>::new(
				|| _my_system,
				|| gamai::empty_node,
			)
		};
		let parent = Node1::<0usize, 0, 0, 0, 0, _, _, _, _, _>::new(
			move || _my_system,
			move || _my_system,
			move || child,
		);
		parent
	};
	let _ = AiBundle::new(tree);
	let _ = AiBundle::new(tree);
	let _ = AiPlugin::new(tree);
	let _ = AiPlugin::new(tree);
}
// define_node!(0);
// define_node!(1);
