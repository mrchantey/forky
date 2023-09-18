use bevy::prelude::*;

// pub trait NodeSets: Send + Sync + Clone + 'static {
pub trait NodeSets {
	fn child_edge_set(&self) -> impl SystemSet;
	fn node_set(&self) -> impl SystemSet;
	fn child_node_set(&self) -> impl SystemSet;
}
