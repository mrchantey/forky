use crate::camera::OrbitController;
use crate::maze::*;
use bevy::prelude::*;

pub fn adjust_camera_on_respawn(
	mut respawn_event: EventReader<RespawnEvent>,
	mut query: Query<(Entity, &mut Transform, &mut OrbitController)>,
) {
	for e in respawn_event.read() {
		for (_entity, mut transform, mut controller) in query.iter_mut() {
			let dist = 1. + usize::max(e.num_rows, e.num_cols) as f32 * 3.;
			transform.translation.x = 0.;
			transform.translation.z = dist;
			transform.translation.y = dist;
			transform.look_at(Vec3::ZERO, Vec3::Y);
			// commands.entity(entity).insert(controller.ease_to(
			// 	OrbitController {
			// 		radius: 10000.,
			// 		..default()
			// 	},
			// 	EaseFunction::SineInOut,
			// 	EasingType::Once {
			// 		duration: Duration::from_secs_f32(4.),
			// 	},
			// ));
			controller.radius = dist;
		}
	}
}
