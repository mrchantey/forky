use super::*;
use crate::maze::*;
use bevy::prelude::*;
use forky_core::{math::*, *};

#[derive(Component)]
pub struct RectMazeSpatial {
	pub rect_maze: RectMaze,
	pub cell_width: f32,
	pub wall_width: f32,
	pub wall_height: f32,
	// pub cols: usize,
	// pub rows: usize,
}

impl Default for RectMazeSpatial {
	fn default() -> Self {
		let rect_maze = RectMaze::default();
		Self {
			rect_maze,
			cell_width: 1.,
			wall_width: 0.2,
			wall_height: 0.5,
		}
	}
}

impl RectMazeSpatial {
	pub fn cols(&self) -> usize { self.rect_maze.num_cols }
	pub fn rows(&self) -> usize { self.rect_maze.num_cols }
	pub fn new(
		num_cols: usize,
		num_rows: usize,
		cell_width: f32,
		wall_width: f32,
		wall_height: f32,
	) -> RectMazeSpatial {
		RectMazeSpatial {
			rect_maze: RectMaze::new(num_cols, num_rows),
			cell_width,
			wall_width,
			wall_height,
		}
	}

	pub fn transforms(&self) -> Vec<(Transform, Vec<Transform>)> {
		let num_rows = self.rect_maze.num_rows;
		let num_cols = self.rect_maze.num_cols;

		let mut trans = Vec::new();
		let grid = self.rect_maze.draw_maze();

		let total_walls_w = f(num_cols + 1) * self.cell_width;
		let total_walls_h = f(num_rows + 1) * self.cell_width;

		let x_min = total_walls_w / 2. - self.cell_width / 2.;
		let z_min = total_walls_h / 2. - self.cell_width / 2.;

		for row in 0..num_rows + 1 {
			for col in 0..num_cols + 1 {
				let i = col + row * (num_cols + 1);
				let func = mesh_shape::from_u8(grid[i]);
				let mut tt = func(self.cell_width, self.wall_width);
				let x = -x_min + self.cell_width * f(col);
				let z = -z_min + self.cell_width * f(row);
				tt.0.translation.x += x;
				tt.0.translation.z += z;
				trans.push(tt);
			}
		}
		trans
	}
}
