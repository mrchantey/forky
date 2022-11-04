use crate::{maze::mesh_shape, *};
use bevy::prelude::*;
use forky_core::{math::*, *};

use super::*;

pub fn spawn(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let num_cols = 10;
	let num_rows = 10;
	let cell_width = 2.;
	let h_cell_width = cell_width / 2.;
	let wall_width = 0.2;
	let wall_height = 0.5;


	let mut maze = RectMaze::new(num_cols, num_rows);

	maze.depth_first_backtrack(|f| {});


	let walls = maze.transforms(cell_width, wall_width);
	let walls = maze_wall::spawn_all(
		&mut commands,
		&mut meshes,
		&mut materials,
		&walls,
		wall_height,
	);


	let floor_h = 0.2;
	let floor_w = num_cols as f32 * cell_width + wall_width;
	let floor_d = num_rows as f32 * cell_width + wall_width;
	let floor = commands
		.spawn_bundle(PbrBundle {
			transform: Transform::from_position_y(-floor_h / 2.)
				.with_scale_xyz(floor_w, floor_h, floor_d),
			mesh: meshes.add(Mesh::from(shape::Cube::default())),
			material: materials.add(Color::rgb(1., 1., 1.).into()),
			..default()
		})
		.id();

	let board = commands
		.spawn_bundle(SpatialBundle::default())
		.push_children(&[walls, floor])
		.id();

	let ball = commands
		.spawn_bundle(PbrBundle {
			transform: Transform::from_xyz(
				-h_cell_width,
				h_cell_width,
				-h_cell_width,
			)
			.with_scale_value(h_cell_width * 0.5),
			mesh: meshes.add(Mesh::from(shape::Icosphere::default())),
			material: materials.add(Color::rgb(1., 1., 1.).into()),
			..default()
		})
		.id();
}
