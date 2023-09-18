use bevy_ecs::prelude::*;

pub trait NodeSets {
	fn child_edge_set(&self) -> impl SystemSet;
	fn node_set(&self) -> impl SystemSet;
	fn child_node_set(&self) -> impl SystemSet;
}
