use super::*;

pub trait IntoRootNode {
	type Out: AiNode;
	fn into_root_node(self) -> Self::Out;
}

pub trait IntoChildNode<
	const GRAPH_ID: usize,
	const GRAPH_DEPTH: usize,
	const CHILD_INDEX: usize,
	const NODE_ID: usize,
	const PARENT_DEPTH: usize,
>: 'static + Send + Sync + Sized
{
	type Out: AiNode;
	fn into_child_node(self) -> Self::Out;
}

// implement for builders
impl<F, T> IntoRootNode for F
where
	T: IntoRootNode,
	F: FnOnce() -> T,
{
	type Out = T::Out;
	fn into_root_node(self) -> Self::Out { self().into_root_node() }
}

// implement for builders
impl<
		const GRAPH_ID: usize,
		const GRAPH_DEPTH: usize,
		const CHILD_INDEX: usize,
		const NODE_ID: usize,
		const PARENT_DEPTH: usize,
		F,
		T,
	> IntoChildNode<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, PARENT_DEPTH> for F
where
	T: IntoChildNode<GRAPH_ID, GRAPH_DEPTH, CHILD_INDEX, NODE_ID, PARENT_DEPTH>,
	F: 'static + Send + Sync + FnOnce() -> T,
{
	type Out = T::Out;
	fn into_child_node(self) -> Self::Out { self().into_child_node() }
}
