use bevy_derive::Deref;
use bevy_derive::DerefMut;
use bevy_ecs::prelude::*;
use petgraph::graph::DiGraph;

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct EntityGraph(pub DiGraph<Entity, ()>);
