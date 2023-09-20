use bevy_ecs::prelude::*;

#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
pub struct BeforeNodeSet<const GRAPH_ID: usize, const DEPTH: usize>;
#[derive(SystemSet, Debug, Clone, Eq, PartialEq, Hash)]
pub struct NodeSet<const GRAPH_ID: usize, const DEPTH: usize>;