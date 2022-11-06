use std::time::Duration;
use super::*;
use crate::{input::OrbitController, *};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use forky_core::{math::*, *};




fn adjust_on_respawn(
	time: Res<Time>,
	mut commands: Commands,
	mut respawn_event: EventReader<maze_3d::RespawnEvent>,
	mut query: Query<(Entity, &Transform), With<OrbitController>>,
) {
	for e in respawn_event.iter() {
		for (entity, tran) in query.iter_mut() {
			let dist = e.num_cols as f32;
			let position = Vec3::new(0., dist, dist);
			commands.entity(entity).insert(pose::PoseLerp {
				timer: Timer::new(Duration::from_secs(2), false),
				origin: Pose::from_transform(tran),
				target: Pose {
					position,
					rotation: Quat::look_away(position),
				},
			});
		}
	}
}
