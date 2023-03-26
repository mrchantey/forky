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
			.add_systems((
				cast_camera_ray, 
				select_entity,
				update_selected,
				on_interact_color_change,
			).chain(),
			)
			.__();
	}
}
