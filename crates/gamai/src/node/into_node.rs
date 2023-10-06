use super::*;

pub trait IntoRootNode {
	type Out: AiNode;
	fn into_root_node(self) -> Self::Out;
}

pub trait IntoChildNode<const CHILD_INDEX: usize, Parent: IntoNodeId>:
	'static + Send + Sync + Sized
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
impl<const CHILD_INDEX: usize, Parent: IntoNodeId, F, T>
	IntoChildNode<CHILD_INDEX, Parent> for F
where
	T: IntoChildNode<CHILD_INDEX, Parent>,
	F: 'static + Send + Sync + FnOnce() -> T,
{
	type Out = T::Out;
	fn into_child_node(self) -> Self::Out { self().into_child_node() }
}