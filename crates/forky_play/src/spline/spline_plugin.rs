use crate::*;
use bevy::prelude::*;

pub struct SplinePlugin;

impl Plugin for SplinePlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_plugins(spline::graph::SplineGraphPlugin)
			.add_plugins(spline::ecs_graph::EcsSplineGraphPlugin)
			.add_plugins(spline::tool::SplineToolPlugin)
			.add_plugins(spline::physics::SplinePhysicsPlugin)
			.__();
	}
}
