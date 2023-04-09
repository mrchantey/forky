use crate::*;
use bevy::prelude::*;

#[derive(Resource)]
pub struct GraphSettings {
	pub start_node: spline::graph::SplineNode,
	pub end_node: spline::graph::SplineNode,
	pub graph_id: spline::ecs_graph::EcsSplineGraphId,
}

impl GraphSettings {
	pub fn new(
		start_node: spline::graph::SplineNode,
		end_node: spline::graph::SplineNode,
		graph_id: spline::ecs_graph::EcsSplineGraphId,
	) -> Self {
		Self {
			start_node,
			end_node,
			graph_id,
		}
	}
}
