#![feature(
	return_position_impl_trait_in_trait,
	associated_const_equality,
	generic_const_exprs
)]
#![allow(incomplete_features)]
//this example is for macro expansion, for usage see the `tests` directory
use gamai::*;

#[node_system]
fn my_system<N: AiNode>() {}
// gamai::define_node!(0);
fn main() {
	let _tree1 = tree! {<my_system/>};
	// let _ = tree1.bundle_inactive();
	// let _ = tree1.plugin();
	// let _tree2 = tree! {
	// 	<my_system>
	// 		<my_system/>
	// 	</my_system>
	// };
}

// struct Foo<T>(pub T);

// impl<T> Foo<T> {
// 	fn new(val: T) -> Self { Self(val) }
// }

// impl AiTree for AutoGenTree0 {
// 	fn get_into_root_node(self) -> impl IntoRootNode {
// 		gamai::Node0::<0usize, 0, 0, 0, 0, _, _, _, _>::new(
// 			|| my_system,
// 			|| gamai::empty_node,
// 		)
// 	}
// 	fn get_into_child_node<
// 		const GRAPH_ID: usize,
// 		const GRAPH_DEPTH: usize,
// 		const CHILD_INDEX: usize,
// 		const NODE_ID: usize,
// 		const PARENT_DEPTH: usize,
// 		// Out: AiNode,
// 	>(
// 		self,
// 	) -> impl IntoIntoChildNode<
// 		GRAPH_ID,
// 		GRAPH_DEPTH,
// 		CHILD_INDEX,
// 		NODE_ID,
// 		PARENT_DEPTH,
// 		// Out,
// 	>
// // where
// 	// 	[(); GRAPH_DEPTH + 1]:,
// 	// 	[(); PARENT_DEPTH + 1]:,
// 	{
// 		Node0::<0, 0, 0, 0, 0, _, _, _, _>::new(
// 			|| my_system,
// 			|| gamai::empty_node,
// 		)
// 		// let a = gamai::Node0::<
// 		// 	GRAPH_ID,
// 		// 	GRAPH_DEPTH,
// 		// 	CHILD_INDEX,
// 		// 	NODE_ID,
// 		// 	PARENT_DEPTH,
// 		// 	_,
// 		// 	_,
// 		// 	_,
// 		// 	_,
// 		// >::new(|| my_system, || gamai::empty_node);
// 	}
// }
