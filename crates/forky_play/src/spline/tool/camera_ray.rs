use super::*;
use crate::*;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier3d::prelude::*;

const MAX_DISTANCE: f32 = 1000.;

#[derive(Resource, Default)]
pub struct CameraRay {
	pub ray: Ray,
	pub point: Vec3,
	pub entity: Option<Entity>,
}

pub fn cast_camera_ray(
	mut commands: Commands,
	camera_query: Query<(&Camera, &GlobalTransform), With<input::ActiveCamera>>,
	windows: Query<&Window>,
	rapier_context: Res<RapierContext>,
) {
	let (camera, camera_transform) = camera_query.single();
	let Some(cursor_position) = windows.single().cursor_position() else { return; };
	let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else { return; };

	if let Some((entity, toi)) = rapier_context.cast_ray(
		ray.origin,
		ray.direction,
		MAX_DISTANCE,
		false,
		QueryFilter::default(),
	) {
		let point = ray.origin + ray.direction * toi;
		commands.insert_resource(CameraRay {
			ray,
			point,
			entity: Some(entity),
		});
	} else {
		commands.insert_resource(CameraRay {
			ray,
			point: ray.origin + ray.direction * 1000.,
			entity: None,
		});
	};
}
