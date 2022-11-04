use crate::{maze::mesh_shape, *};
use bevy::prelude::*;
use forky_core::{math::*, *};
use bevy_rapier3d::prelude::*;

pub fn spawn(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	cell_width:f32,
	wall_width: f32,
	num_cols:usize,
	num_rows:usize,
) -> Entity {

	let floor_h = 0.2;
	let floor_w = num_cols as f32 * cell_width + wall_width;
	let floor_d = num_rows as f32 * cell_width + wall_width;
	commands
		.spawn_bundle(PbrBundle {
			transform: Transform::from_position_y(-floor_h / 2.)
				.with_scale_xyz(floor_w, floor_h, floor_d),
			mesh: meshes.add(Mesh::from(shape::Cube::default())),
			material: materials.add(Color::rgb(1., 1., 1.).into()),
			..default()
		})
		.insert(Collider::cuboid(0.5, 0.5, 0.5))
		.id()
}