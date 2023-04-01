use super::*;
use crate::*;
use bevy::prelude::*;

pub struct SplineToolPlugin;
#[rustfmt::skip]
impl Plugin for SplineToolPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			//
			.init_resource::<CameraRay>()
			.init_resource::<InteractionSettings>()
			.add_startup_system(spawn_resources.in_base_set(StartupSet::PreStartup))
			.add_systems((
				cast_camera_ray, 
				set_entity_intersect,
				select_entities,
				highlight_entities,
				//these can be parallel
				move_items,
				on_interact_state_change,
			).chain(),
			)
			.__();
	}
}
