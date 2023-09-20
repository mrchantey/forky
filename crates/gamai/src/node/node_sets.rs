use bevy_ecs::prelude::*;

#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
pub struct NodePreUpdate<const GRAPH_ID: usize, const DEPTH: usize>;
#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
pub struct NodeUpdate<const GRAPH_ID: usize, const DEPTH: usize>;
#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
pub struct NodePostUpdate<const GRAPH_ID: usize, const DEPTH: usize>;
