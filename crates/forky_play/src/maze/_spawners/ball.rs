use crate::maze::*;
use crate::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use forky_core::{math::*, *};


use super::maze_3d::MazeItemTag;

#[derive(Component)]
pub struct BallTag;

pub fn respawn(
	mut spawn_event: EventReader<maze_3d::RespawnEvent>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	for e in spawn_event.iter() {
		let h_cell_width = e.cell_width / 2.;
		let diameter = h_cell_width * 0.5;
		commands
			.spawn_bundle(PbrBundle {
				transform: Transform::from_xyz(
					-h_cell_width,
					h_cell_width,
					-h_cell_width,
				)
				.with_scale_value(diameter),
				mesh: meshes.add(Mesh::from(shape::Icosphere::default())),
				material: materials.from_white(),
				..default()
			})
			.insert(BallTag)
			.insert(MazeItemTag)
			.insert(RigidBody::Dynamic)
			.insert(Collider::ball(1.))
			.insert(AdditionalMassProperties::Mass(10.))
			.insert(Restitution {
				coefficient: 0.5,
				combine_rule: CoefficientCombineRule::Average,
			})
			.insert(Friction {
				coefficient: 0.,
				combine_rule: CoefficientCombineRule::Min,
			})
			.insert(ColliderDebugColor(Color::RED));
	}
}



const KILL_FLOOR: f32 = -50.;

pub fn despawn_on_ball_fall(
	mut despawn_event: EventWriter<maze_3d::DespawnEvent>,
	query: Query<&Transform, With<ball::BallTag>>,
) {
	for tran in query.iter() {
		if tran.translation.y < KILL_FLOOR {
			despawn_event.send(maze_3d::DespawnEvent);
		}
	}
}
