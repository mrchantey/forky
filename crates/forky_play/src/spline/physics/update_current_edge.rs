use super::*;
use crate::spline::ecs_graph::EcsSplineGraphId;
use crate::spline::ecs_graph::EcsSplineGraphLookup;
use crate::spline::graph::*;
use bevy::prelude::*;


fn update_edge(
	commands: &mut Commands,
	entity: Entity,
	mut position: Mut<SplinePosition>,
	mut edge: Mut<SplineEdge>,
	graph: &SplineGraph,
) {
	//we still need to check in case edge changed or removed
	// if **position >= 0. && **position <= 1. {
	// 	return;
	// }
	let (t, next_edge) = match graph.get_current_edge(&edge, **position) {
		Some(value) => value,
		None => {
			commands.entity(entity).despawn();
			//TODO add velocity with tangent
			return;
		}
	};
	*edge = next_edge;
	position.0 = t;
}


pub fn update_current_edge(
	mut commands: Commands,
	graph_lookup: Res<SplineGraphLookup>,
	mut query: Query<(
		Entity,
		&mut SplinePosition,
		&mut SplineEdge,
		&SplineGraphId,
	)>,
) {
	for (entity, position, edge, graph_id) in query.iter_mut() {
		let graph = graph_lookup.get(&graph_id).unwrap();
		update_edge(&mut commands, entity, position, edge, graph);
	}
}

pub fn update_current_edge_ecs(
	mut commands: Commands,
	graph_lookup: Res<EcsSplineGraphLookup>,
	mut query: Query<(
		Entity,
		&mut SplinePosition,
		&mut SplineEdge,
		&EcsSplineGraphId,
	)>,
) {
	for (entity, position, edge, graph_id) in query.iter_mut() {
		let graph = graph_lookup.get(&graph_id).unwrap();
		update_edge(&mut commands, entity, position, edge, &graph.graph);
	}
}
