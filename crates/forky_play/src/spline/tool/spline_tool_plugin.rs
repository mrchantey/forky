use super::*;
use crate::*;
use bevy::prelude::*;

pub struct SplineToolPlugin;
#[rustfmt::skip]
impl Plugin for SplineToolPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_systems(PreUpdate,(
				create_spline_node,
				link_spline_nodes,
			).in_set(tool::ToolSystemSet::ModifySelection))
			.__();
	}
}
