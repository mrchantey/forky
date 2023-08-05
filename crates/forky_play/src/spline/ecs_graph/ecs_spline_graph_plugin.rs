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
			.configure_set(Update,EcsSplineGraphSet::Update)
			.configure_set(Update,EcsSplineGraphSet::Modify
				.after(EcsSplineGraphSet::Update))
			.add_systems(Update,(
				on_handle_moved,
				on_node_moved,
				on_edge_modified).in_set(EcsSplineGraphSet::Update))
			.add_systems(Update,apply_catmull_rom.in_set(EcsSplineGraphSet::Modify))
			.add_systems(PostUpdate,(
				// spline::utils::draw_spline,
				spline::utils::draw_ecs_graph,
				// spline::utils::draw_graph
			))
			.__();
	}
}
