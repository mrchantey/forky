//this example is used for macro expansion, for usage see the `tests` directory
#![feature(
	return_position_impl_trait_in_trait,
	associated_const_equality,
	generic_const_exprs
)]
#![allow(incomplete_features)]
use gamai::*;

fn main() {
	type Root = TreePathRoot<0>;
	let tree = || {
		let a = Node0::<Root, _>::new(DefaultAttributes::default());
		Node1::<Root, _, _>::new(DefaultAttributes::default(), a).into_root()
	};

	let _ = AiBundle::new(tree);
	let _ = AiBundle::new(tree);
	let _ = AiPlugin::new(tree);
	let _ = AiPlugin::new(tree);
}
