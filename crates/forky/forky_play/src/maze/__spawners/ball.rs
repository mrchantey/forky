use crate::maze::*;
use crate::*;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use forky_core::*;

#[derive(Component)]
pub struct BallTag;

pub fn respawn(
	mut spawn_event: EventReader<RespawnEvent>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	for e in spawn_event.iter() {
		let h_cell_width = e.cell_width / 2.;
		let diameter = h_cell_width * 0.5;
		let start_offset = tern!(e.num_cols % 2 == 0;-h_cell_width;0.);

		commands
			.spawn(PbrBundle {
				transform: Transform::from_xyz(
					start_offset,
					h_cell_width,
					start_offset,
				)
				.with_scale_value(diameter),
				mesh: meshes.add(Mesh::from(shape::UVSphere::default())),
				material: materials.add(StandardMaterial {
					metallic: 0.5,
					perceptual_roughness: 0.5,
					// roughness:1.,
					reflectance: 1.,
					base_color: Color::WHITE,
					..default()
				}),
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
	mut commands: Commands,
	mut despawn_event: EventWriter<DespawnEvent>,
	query: Query<(Entity, &Transform), (With<ball::BallTag>, Without<DeadTag>)>,
) {
	for (eid, tran) in query.iter() {
		if tran.translation.y < KILL_FLOOR {
			despawn_event.send(DespawnEvent::Success);
			commands.entity(eid).insert(DeadTag);
		}
	}
}
