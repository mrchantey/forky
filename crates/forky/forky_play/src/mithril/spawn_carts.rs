use super::*;
use crate::spline::{
	ecs_graph::EcsSplineGraphLookup, physics::SplinePhysicsBundle,
};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Cart;
#[derive(Resource, Deref, DerefMut)]
pub struct LastCartSpawn(pub Duration);


pub fn spawn_carts(
	mut commands: Commands,
	node_settings: Res<GraphSettings>,
	cart_settings: Res<CartSettings>,
	time: Res<Time>,
	mut last_cart_spawn: ResMut<LastCartSpawn>,
	graph_lookup: Res<EcsSplineGraphLookup>,
	query: Query<&Cart>,
) {
	let num_carts = query.iter().count();

	if num_carts >= cart_settings.max_carts {
		return;
	}

	if time.elapsed() < **last_cart_spawn + cart_settings.cart_spawn_interval {
		return;
	}

	*last_cart_spawn = LastCartSpawn(time.elapsed());

	let graph = graph_lookup.get(&*node_settings.graph_id).unwrap();

	let (_, _, edge) =
		graph.graph.edges(node_settings.start_node).next().unwrap();

	commands.spawn((
		SplinePhysicsBundle::new(
			cart_settings.mesh.clone(),
			cart_settings.material.clone(),
			edge.clone(),
			node_settings.graph_id,
		)
		.with_friction(0.),
		Cart,
	));
}
