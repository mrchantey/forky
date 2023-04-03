use super::*;
use crate::*;
use bevy::prelude::*;

pub struct EcsSplineGraphPlugin;


#[rustfmt::skip]
impl Plugin for EcsSplineGraphPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.insert_resource(EcsSplineGraphLookup::new())
			.add_system(on_handle_moved)
			.add_system(on_node_moved)
			.__();
	}
}
