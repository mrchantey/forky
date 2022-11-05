use crate::{maze::mesh_shape, *};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use forky_core::{math::*, *};

use super::RectMazeSpatial;


pub fn spawn(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	maze: &RectMazeSpatial,
) -> Entity {
	let floor_h = maze.wall_height / 2.;
	let floor_w = maze.cols() as f32 * maze.cell_width + maze.wall_width;
	let floor_d = maze.rows() as f32 * maze.cell_width + maze.wall_width;
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
