use super::*;
use crate::*;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier3d::prelude::*;

const MAX_DISTANCE: f32 = 1000.;

pub fn select_entity(
	mut commands: Commands,
	camera_ray: Res<CameraRay>,
	mouse: Res<Input<MouseButton>>,
	interactable_query: Query<(Entity), With<InteractColor>>,
) {
	if let Some(entity) = camera_ray.entity {
		if mouse.pressed(SELECT_BUTTON) {
			commands
				.entity(entity)
				.insert(InteractColor(SELECT_COLOR))
				.insert(Selected {});
		} else {
			commands
				.entity(entity)
				.insert(InteractColor(HOVER_COLOR))
				.remove::<Selected>();
		}
	} else {
		for interactable in interactable_query.iter() {
			commands
				.entity(interactable)
				.insert(InteractColor(INACTIVE_COLOR))
				.remove::<Selected>();
		}
	};
}


pub fn update_selected(
	camera_ray: Res<CameraRay>,
	keys: Res<Input<KeyCode>>,
	mut query: Query<(&mut Transform), With<Selected>>,
) {
	for mut transform in query.iter_mut() {
		if (keys.just_pressed(KeyCode::PageUp)) {
			transform.translation.y += 0.1;
		}
		if (keys.just_pressed(KeyCode::PageDown)) {
			transform.translation.y -= 0.1;
		}
		if let Some(distance) = camera_ray
			.ray
			.intersect_plane(transform.translation, Vec3::UP)
		{
			let point = camera_ray.ray.get_point(distance);
			transform.translation = point;
		}
	}
}
