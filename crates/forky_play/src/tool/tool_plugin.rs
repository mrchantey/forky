use super::*;
use crate::*;
use bevy::prelude::*;

pub struct ToolPlugin;
#[rustfmt::skip]
impl Plugin for ToolPlugin {
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
			).chain())//todo in preupdate
			.add_systems((
				move_selected_interactables,
				set_interactable_colors,
			).after(highlight_entities))
			//TODO in sets
			.add_systems((
				append_interactable_mesh,
			).after(set_interactable_colors))
			.__();
	}
}
