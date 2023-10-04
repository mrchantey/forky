use super::*;
// use bevy_ecs::prelude::*;

// pub trait IntoNode<Node: AiNode>: 'static + Send + Sync + Sized {
// 	fn into_node<const CHILD_INDEX: usize, Parent: IntoNodeId>(&self) -> Node;
// pub trait IntoNode: 'static + Send + Sync + Sized {
// 	type Out<const CHILD_INDEX:usize,Parent:IntoNodeId>:AiNode;
// 	fn into_node<const CHILD_INDEX: usize, Parent: IntoNodeId>(
// 		&self,
// 	) -> Self::Out<CHILD_INDEX, Parent>;
// }
pub trait IntoNode<const CHILD_INDEX: usize, Parent: IntoNodeId>:
	'static + Send + Sync + Sized
{
	type Out: AiNode;
	fn into_node(self) -> Self::Out;
}

// impl<const CHILD_INDEX: usize, Parent: IntoNodeId, F, T>
// 	IntoNode<CHILD_INDEX, Parent> for F
// where
// 	T: IntoNode<CHILD_INDEX, Parent>,
// 	F: 'static + Send + Sync + Fn() -> T,
// {
// 	type Out = T::Out;
// 	fn into_node(self) -> Self::Out { self().into_node() }
// }

// fn node_state(self, world: &World, entity: Entity) -> Option<NodeState> {
// 	world
// 		.entity(entity)
// 		.get::<DerefNodeState<Node>>()
// 		.map(|s| **s)
// }
// fn edge_state(self, world: &World, entity: Entity) -> Option<EdgeState> {
// 	world
// 		.entity(entity)
// 		.get::<DerefEdgeState<Node>>()
// 		.map(|s| **s)
// }

// impl<F, Node> IntoNode<Node> for F
// where
// 	Node: AiNode,
// 	F: 'static + Send + Sync + Fn() -> Node,
// {
// }
