//this example is used for macro expansion, for usage see the `tests` directory
#![feature(return_position_impl_trait_in_trait, associated_const_equality)]
use gamai::*;
fn main() {}
define_node!(0);
define_node!(1);

fn my_system() {}

fn foobar() {
	let child =
		Node0::<0, RootParent<0>, _, _, _, _>::new(|| my_system, || my_system);
	let parent = Node1::<0, RootParent<0>, _, _, _, _, _>::new(
		|| my_system,
		|| my_system,
		child,
	);
}
