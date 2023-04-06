use super::*;
use crate::*;
use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum EcsSplineGraphSet {
	Update,
	Modify,
}



pub struct EcsSplineGraphPlugin;

#[rustfmt::skip]
impl Plugin for EcsSplineGraphPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.insert_resource(EcsSplineGraphLookup::new())
			.configure_set(EcsSplineGraphSet::Update
				.in_base_set(CoreSet::Update))
			.configure_set(EcsSplineGraphSet::Modify
				.in_base_set(CoreSet::Update)
				.after(EcsSplineGraphSet::Update))
			.add_systems((
				on_handle_moved,
				on_node_moved,
				on_edge_modified)
				.in_set(EcsSplineGraphSet::Update))
			.add_system(apply_catmull_rom
				.in_set(EcsSplineGraphSet::Modify))
			.add_systems((
				// spline::utils::draw_spline,
				spline::utils::draw_ecs_graph,
				// spline::utils::draw_graph
			)
				.in_base_set(CoreSet::PostUpdate))
			.__();
	}
}
