use bevy::prelude::*;

// pub trait NodeSets: Send + Sync + Clone + 'static {
pub trait NodeSets {
	fn edge_set(&self) -> impl SystemSet;
	fn node_set(&self) -> impl SystemSet;
	fn action_set(&self) -> impl SystemSet;
}
