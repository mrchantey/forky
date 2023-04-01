use super::*;
use crate::*;
use bevy::prelude::*;

pub struct SplinePlugin;


#[rustfmt::skip]
impl Plugin for SplinePlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_system(graph::on_node_moved)
			.init_resource::<graph::SplineGraphLookup>()
			.__();
	}
}
