#![feature(associated_const_equality, generic_const_exprs)]
#![allow(incomplete_features)]
//this example is for macro expansion, for usage see the `tests` directory
use gamai::*;

#[action]
fn my_system<N: AiNode>() {}

fn main() {
	let el = || {
		ParentElement1::<TreePathRoot<0>, _, _, _>::new(
			my_system,
			my_system,
			ParentElement0::<TreePathRoot<0>, _, _>::new(my_system, my_system),
		)
		.as_root()
	};

	// let tree1 = || tree! {<my_system/>};
	let _bundle = TreeBundle::new(el);
	let _bundle = TreeBundle::new(el);
	// let _plugin = TreePlugin::new(tree1);
	// let _plugin = TreePlugin::new(tree1);
}
