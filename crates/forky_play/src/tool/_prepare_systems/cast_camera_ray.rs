use crate::tool::*;
use crate::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const MAX_DISTANCE: f32 = 1000.;


pub fn cast_camera_ray(
	mut commands: Commands,
	mut camera_ray: ResMut<CameraRay>,
	mouse: Res<Input<MouseButton>>,
	camera_query: Query<(&Camera, &GlobalTransform), With<input::ActiveCamera>>,
	hovered: Query<Entity, With<Hovered>>,
	primary_interacted: Query<Entity, With<PrimaryInteracted>>,
	windows: Query<&Window>,
	rapier_context: Res<RapierContext>,
) {
	let (camera, camera_transform) = camera_query.single();
	let Some(cursor_position) = windows.single().cursor_position() else { return; };
	let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else { return; };

	camera_ray.ray = ray;

	camera_ray.ground_intersect = RayIntersect::from_plane(
		ray,
		Vec3::ZERO,
		Vec3::UP,
		&camera_ray.ground_intersect,
	);

	for entity in hovered.iter() {
		commands.entity(entity).remove::<Hovered>();
	}

	//dont cast ray if dragging
	if mouse.pressed(SELECT_BUTTON) {
		return;
	}
	for entity in primary_interacted.iter() {
		commands.entity(entity).remove::<PrimaryInteracted>();
	}
	camera_ray.entity = if let Some((entity, _dist)) = rapier_context.cast_ray(
		ray.origin,
		ray.direction,
		MAX_DISTANCE,
		false,
		QueryFilter::default(),
	) {
		commands.entity(entity).insert((Hovered, PrimaryInteracted));
		Some(entity)
	} else {
		None
	};
}
pub fn set_entity_intersect(
	mut camera_ray: ResMut<CameraRay>,
	query: Query<&Transform, With<PrimaryInteracted>>,
) {
	// camera_ray.entity
	camera_ray.entity_intersect =
		if let Ok(first_transform) = query.get_single() {
			RayIntersect::from_plane(
				camera_ray.ray,
				first_transform.translation,
				Vec3::UP,
				&camera_ray.entity_intersect,
			)
		} else {
			None
		};
}
