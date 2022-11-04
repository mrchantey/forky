use super::*;
use bevy::prelude::*;
use bevy_rapier3d::na::Point;
use forky_core::math::*;

pub fn spawn_basic_scene(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Quad {
			size: Vec2::new(2., 2.),
			..default()
		})),
		transform: Transform::from_xyz(0., 1., 0.)
			.with_rotation(Quat::from_rotation_y(TAU / 2.)),
		// mesh: meshes.add(Mesh::from(shape::Plane { size: 10. })),
		material: materials.add(Color::rgb(0.2, 1., 0.2).into()),
		..default()
	});
}
pub fn spawn_lights(mut commands: Commands) {
	commands.spawn_bundle(PointLightBundle {
		transform: Transform::from_xyz(-5., 5., 3.),
		point_light: PointLight {
			intensity: 1000.,
			color: Color::FUCHSIA,
			..default()
		},
		..default()
	});
	commands.spawn_bundle(PointLightBundle {
		transform: Transform::from_xyz(3., 5., -5.),
		point_light: PointLight {
			intensity: 1000.,
			color: Color::CYAN,
			..default()
		},
		..default()
	});
}
