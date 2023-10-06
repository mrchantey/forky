//this example is used for macro expansion, for usage see the `tests` directory
#![feature(return_position_impl_trait_in_trait, associated_const_equality)]
use gamai::*;
fn main() {}
define_node!(0);
define_node!(1);

fn _my_system() {}

fn _foobar() {
	let tree = || {
		let child = || {
			Node0::<0, RootParent<0>, _, _, _, _>::new(
				|| _my_system,
				|| _my_system,
			)
		};
		let parent = Node1::<0, RootParent<0>, _, _, _, _, _>::new(
			move || _my_system,
			move || _my_system,
			move || child,
		);
		parent
	};
	let _ = AiBundle::inactive(tree);
	let _ = AiBundle::inactive(tree);
	let _ = AiPlugin::new(tree);
	let _ = AiPlugin::new(tree);
}
