use super::*;


// pub trait IntoChildNodeOpaque{
// 	fn into_child_node_opaque()->impl AiNode;
// }

pub trait IntoDepth {
	type Out<const NEW_DEPTH: usize>: IntoDepth;
	fn into_depth<const NEW_DEPTH: usize>(self) -> Self::Out<NEW_DEPTH>;
	// fn into_depth(self) -> Self::NewDepth { Self::NewDepth::default() }
	// type IntoDepth<const NEW_DEPTH: usize>: NodeDepth<NEW_DEPTH, Marker>;
	// fn into_depth<const NEW_DEPTH: usize>() -> impl NodeDepth<NEW_DEPTH>;
}

// pub trait IntoIntoChildNode<
// 	const GRAPH_ID: usize,
// 	const GRAPH_DEPTH: usize,
// 	const CHILD_INDEX: usize,
// 	const NODE_ID: usize,
// 	const PARENT_DEPTH: usize,
// >:
// 	AiNode
// 	+ IntoChildNode<
// 		GRAPH_ID,
// 		GRAPH_DEPTH,
// 		CHILD_INDEX,
// 		NODE_ID,
// 		PARENT_DEPTH,
// 		Self,
// 	>
// {
// }

// impl<
// 		const GRAPH_ID: usize,
// 		const GRAPH_DEPTH: usize,
// 		const CHILD_INDEX: usize,
// 		const NODE_ID: usize,
// 		const PARENT_DEPTH: usize,
// 		T: AiNode
// 			+ IntoChildNode<
// 				GRAPH_ID,
// 				GRAPH_DEPTH,
// 				CHILD_INDEX,
// 				NODE_ID,
// 				PARENT_DEPTH,
// 				Self,
// 			>,
// 	> IntoIntoChildNode<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, PARENT_DEPTH>
// 	for T
// {
// }

pub trait DefaultNode<Out> {
	fn into_default() -> Out;
}

pub trait IntoRootNode {
	type Out: AiNode;
	fn into_root_node(self) -> Self::Out;
}

pub trait IntoChildNode<
	const GRAPH_ID: usize,
	const GRAPH_DEPTH: usize,
	const CHILD_INDEX: usize,
	const NODE_ID: usize,
	Out,
>: 'static + Send + Sync + Sized
{
	// type Out: AiNode;
	fn into_child_node(self) -> Out;
	// fn into_child_node(self) -> Self::Out;
}

// implement for builders, essential because closures are required for passing instances
impl<F, T> IntoRootNode for F
where
	T: IntoRootNode,
	F: FnOnce() -> T,
{
	type Out = T::Out;
	fn into_root_node(self) -> Self::Out { self().into_root_node() }
}

// implement for builders, essential because closures are required for passing instances
// impl<
// 		const GRAPH_ID: usize,
// 		const GRAPH_DEPTH: usize,
// 		const CHILD_INDEX: usize,
// 		const NODE_ID: usize,
// 		const PARENT_DEPTH: usize,
// 		F,
// 		T,
// 		M,
// 	> IntoChildNode<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, PARENT_DEPTH, M>
// 	for F
// where
// 	T: IntoChildNode<
// 		GRAPH_ID,
// 		GRAPH_DEPTH,
// 		CHILD_INDEX,
// 		NODE_ID,
// 		PARENT_DEPTH,
// 		M,
// 	>,
// 	F: 'static + Send + Sync + FnOnce() -> T,
// {
// 	type Out = T::Out;
// 	fn into_child_node(self) -> Self::Out { self().into_child_node() }
// }
