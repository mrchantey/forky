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
	let cell_width = 0.2;
	// let h_cell_width = cell_width / 2.;
	let wall_width = 0.02;
	let wall_height = 0.05;

	let mut maze = RectMazeSpatial::new(
		num_cols,
		num_rows,
		cell_width,
		wall_width,
		wall_height,
	);
	maze.rect_maze.depth_first_backtrack(|f| {});

	let board =
		board::spawn(&mut commands, &mut meshes, &mut materials, &maze).id();

	let ball =
		ball::spawn(&mut commands, &mut meshes, &mut materials, &maze).id();
}
