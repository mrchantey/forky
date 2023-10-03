use super::*;
use bevy_ecs::prelude::*;

pub trait IntoNode<Node: AiNode>: 'static + Send + Sync + Sized {
	// fn into_node(&self) -> Node;
	fn node_state(self, world: &World, entity: Entity) -> Option<NodeState> {
		world
			.entity(entity)
			.get::<DerefNodeState<Node>>()
			.map(|s| **s)
	}
	fn edge_state(self, world: &World, entity: Entity) -> Option<EdgeState> {
		world
			.entity(entity)
			.get::<DerefEdgeState<Node>>()
			.map(|s| **s)
	}
}

impl<F, Node> IntoNode<Node> for F
where
	Node: AiNode,
	F: 'static + Send + Sync + Fn() -> Node,
{
}