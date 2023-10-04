//this example is used for macro expansion, for usage see the `tests` directory
#![feature(return_position_impl_trait_in_trait, associated_const_equality)]
use gamai::*;
fn main() {}
define_node!(0);
define_node!(1);

fn _my_system() {}

fn _foobar() {
	let child =
		Node0::<0, RootParent<0>, _, _, _, _>::new(|| _my_system, || _my_system);
	let _parent = Node1::<0, RootParent<0>, _, _, _, _, _>::new(
		|| _my_system,
		|| _my_system,
		child,
	);
}
