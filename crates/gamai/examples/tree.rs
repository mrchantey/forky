#![feature(associated_const_equality, generic_const_exprs)]
#![allow(incomplete_features)]
//this example is for macro expansion, for usage see the `tests` directory
use gamai::*;

#[action(props = 9u32)]
fn my_system<N: AiNode>() {}

fn main() {
	// let tree1 = || tree! {<my_system apply_deferred/>};
	// let _tree3 = tree! {<my_system/><my_system/>};
	let _tree = tree! {<group actions={tree!{<my_system/>}}/>};
	// let tree4 = || {
	// 	tree! {
	// 		<group>
	// 			<group actions={tree!{<my_system/>}}/>
	// 			// <group actions=(my_system)/>
	// 		</group>
	// 	}
	// };
	// let _bundle = TreeBundle::inactive(tree4);
	// let tree = PropTree::new(el, world, entity)

	// let _plugin = TreePlugin::new(tree1);
}


//(my_system), (my_system)).into_root(),

// {
// (gamai::node::ParentElement0::<
// gamai::node::TreePathRoot<5usize>,
// _,
// _,
// >::new(
// my_system.into_action_config(),
// my_system.into_action_config(),
// )
// .into_root())
// },
// {
// (gamai::node::ParentElement0::<
// gamai::node::TreePathRoot<6usize>,
// _,
// _,
// >::new(
// my_system.into_action_config(),
// my_system.into_action_config(),
// )
// .into_root())
// },
