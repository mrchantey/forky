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

// impl<T, Node> IntoNode<Node> for T
// where
// 	T: Fn() -> Node + 'static + Send + Sync,
// 	Node: AiNode,
// {
// 	fn into_node(&self) -> Node { self() }
// }

// #[derive(Component)]
// struct Bar;

// fn foobar(mut query: Query<&mut Bar>) {
// 	for mut item in query.iter_mut() {
// 		*item = Bar;
// 	}
// }
