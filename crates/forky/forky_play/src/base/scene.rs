use crate::*;
use bevy::prelude::*;
use forky_core::math::*;

pub fn spawn_basic_scene(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn(PbrBundle {
		mesh: meshes.add(Mesh::from(Rectangle::from_size(Vec2::splat(2.)))),
		transform: Transform::from_xyz(0., 1., 0.)
			.with_rotation(Quat::from_rotation_y(TAU / 2.)),
		material: materials.from_rgb(0.2, 1., 0.2),
		..default()
	});
}
