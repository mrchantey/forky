use crate::prelude::*;
use bevy_ecs::prelude::*;
use bevy_utils::HashSet;
use std::fmt::Debug;

/// Indicate this node should stop running.
/// As this is frequently added and removed, it is `SparseSet`.
#[derive(Default, Debug, Component, PartialEq)]
#[component(storage = "SparseSet")]
pub struct Interrupt;

pub fn sync_interrupts(
	mut commands: Commands,
	interrupts: Query<Entity, Added<Interrupt>>,
	edges: Query<&Edges>,
) {
	let mut visited = HashSet::default();
	for entity in interrupts.iter() {
		remove_running_recursive(&mut commands, entity, &edges, &mut visited);
	}
}


fn remove_running_recursive(
	commands: &mut Commands,
	entity: Entity,
	edge_query: &Query<&Edges>,
	visited: &mut HashSet<Entity>,
) {
	if visited.contains(&entity) {
		return;
	}
	visited.insert(entity);
	commands.entity(entity).remove::<(Running, RunResult)>();
	if let Ok(edges) = edge_query.get(entity) {
		for edge in edges.iter() {
			remove_running_recursive(commands, *edge, edge_query, visited);
		}
	}
}
