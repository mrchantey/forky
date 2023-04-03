use crate::*;
use bevy::prelude::*;

pub struct SplinePlugin;

impl Plugin for SplinePlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_plugin(spline::graph::SplineGraphPlugin)
			.add_plugin(spline::ecs_graph::EcsSplineGraphPlugin)
			.add_plugin(spline::tool::SplineToolPlugin)
			.add_plugin(spline::physics::SplinePhysicsPlugin)
			.__();
	}
}
