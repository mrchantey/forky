use bevy::prelude::*;
use forky_core::math::*;
use super::*;

pub fn spawn_basic_scene(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands
		.spawn_bundle(PbrBundle {
			mesh: meshes.add(Mesh::from(shape::Cube { size: 1. })),
			material: materials.add(Color::rgb(0., 1., 1.).into()),
			transform: Transform::from_xyz(0., 1., 0.),
			..default()
		})
		.insert(CompanionCube);

	commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Quad {
			size: Vec2::new(2., 2.),
			..default()
		})),
		transform: Transform::from_xyz(0., 1., 0.).with_rotation(Quat::from_rotation_y(TAU / 2.)),
		// mesh: meshes.add(Mesh::from(shape::Plane { size: 10. })),
		material: materials.add(Color::rgb(0.2, 1., 0.2).into()),
		..default()
	});

	commands.spawn_bundle(PointLightBundle {
		transform: Transform::from_xyz(1., 3., -3.),
		..default()
	});
}
