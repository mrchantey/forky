#![feature(associated_const_equality, generic_const_exprs)]
#![allow(incomplete_features)]
//this example is for macro expansion, for usage see the `tests` directory
use gamai::*;

#[action(apply_deferred)]
fn my_system<N: AiNode>() {}

fn main() {
	let tree1 = || tree! {<my_system apply_deferred/>};
	// let _tree2 = || {
	// 	tree! {
	// 			<group actions={
	// 				tree!{
	// 					<my_system/>
	// 					<my_system/>
	// 				}
	// 			}/>
	// 	}
	// };
	let _tree3 = tree! {<my_system/><my_system/>};
	let _tree4 = tree! {
		<group actions={tree!{<my_system/>}}/>
		<group actions=(my_system)/>
	};
	// let _tree2 = || tree! {<group actions=(my_system,my_system)/>};
	let _bundle = TreeBundle::new(tree1);
	let _plugin = TreePlugin::new(tree1);
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
