use crate::*;
use bevy::prelude::*;
use forky_core::math::*;

pub fn spawn_basic_scene(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Quad {
			size: Vec2::new(2., 2.),
			..default()
		})),
		transform: Transform::from_xyz(0., 1., 0.)
			.with_rotation(Quat::from_rotation_y(TAU / 2.)),
		// mesh: meshes.add(Mesh::from(shape::Plane { size: 10. })),
		material: materials.from_rgb(0.2, 1., 0.2),
		..default()
	});
}
pub fn spawn_lights(mut commands: Commands) {
	commands.spawn(PointLightBundle {
		transform: Transform::from_xyz(-5., 5., 3.),
		point_light: PointLight {
			intensity: 1000.,
			color: Color::FUCHSIA,
			shadows_enabled: true,
			..default()
		},
		..default()
	});
	commands.spawn(PointLightBundle {
		transform: Transform::from_xyz(3., 5., -5.),
		point_light: PointLight {
			intensity: 1000.,
			shadows_enabled: true,
			color: Color::CYAN,
			..default()
		},
		..default()
	});
}
