//this example is used for macro expansion, for usage see the `tests` directory
#![feature(
	return_position_impl_trait_in_trait,
	associated_const_equality,
	generic_const_exprs
)]
#![allow(incomplete_features)]
fn main() {}
gamai::define_node!(0);
gamai::define_node!(1);


// impl<
// 		const GRAPH_ID: usize,
// 		const GRAPH_DEPTH: usize,
// 		const CHILD_INDEX: usize,
// 		const NODE_ID: usize,
// 		const PARENT_DEPTH: usize,
// 		const NEW_GRAPH_ID: usize,
// 		const NEW_GRAPH_DEPTH: usize,
// 		const NEW_CHILD_INDEX: usize,
// 		const NEW_NODE_ID: usize,
// 		const NEW_PARENT_DEPTH: usize,
// 		NodeSystem: IntoNodeSystem<NodeSystemMarker>,
// 		NodeSystemMarker: 'static + Send + Sync,
// 		EdgeSystem: IntoNodeSystem<EdgeSystemMarker>,
// 		EdgeSystemMarker: 'static + Send + Sync,
// 	> IntoIntoChildNode<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, PARENT_DEPTH>
// 	for Node0<
// 		GRAPH_ID,
// 		GRAPH_DEPTH,
// 		CHILD_INDEX,
// 		NODE_ID,
// 		PARENT_DEPTH,
// 		NodeSystem,
// 		NodeSystemMarker,
// 		EdgeSystem,
// 		EdgeSystemMarker,
// 	>
// {
// }
