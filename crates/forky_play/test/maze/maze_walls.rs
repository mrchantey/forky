use bevy::prelude::*;
use forky_core::{math::*, *};
use forky_play::{
	maze::{maze_wall, mesh_shape},
	*,
};
use sweet::*;

sweet! {
	it skip "works" {
		app::init()
			// .add_plugin(maze::MazePlugin)
			.add_startup_system(spawn_all_walls)
			.add_plugin(input::DebugCameraPlugin)
			.add_startup_system(utility::surrender_focus)
			// .forky_exit_after(2)
			.run();
	}
}


pub fn spawn_all_walls(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let cell_width = 2.;
	let wall_width = 0.2;

	let mut walls = [
		[
			mesh_shape::horizontal(cell_width, wall_width),
			mesh_shape::vertical(cell_width, wall_width),
			mesh_shape::cross(cell_width, wall_width),
			mesh_shape::none(cell_width, wall_width),
		],
		[
			mesh_shape::top_left(cell_width, wall_width),
			mesh_shape::top_right(cell_width, wall_width),
			mesh_shape::bottom_left(cell_width, wall_width),
			mesh_shape::bottom_right(cell_width, wall_width),
		],
		[
			mesh_shape::left_tee(cell_width, wall_width),
			mesh_shape::top_tee(cell_width, wall_width),
			mesh_shape::right_tee(cell_width, wall_width),
			mesh_shape::bottom_tee(cell_width, wall_width),
		],
		[
			mesh_shape::horizontal_left(cell_width, wall_width),
			mesh_shape::vertical_top(cell_width, wall_width),
			mesh_shape::horizontal_right(cell_width, wall_width),
			mesh_shape::vertical_bottom(cell_width, wall_width),
		],
	];
	let h_cols = (walls.len() as f32 * cell_width)  / 2. - cell_width / 2.;
	let h_rows = (walls[0].len() as f32 * cell_width) / 2. - cell_width / 2.;

	for (col, mut arr) in walls.iter_mut().enumerate() {
		for (row, mut wall) in arr.iter_mut().enumerate() {
			wall.0.translation.x += -h_rows + row as f32 * cell_width;
			wall.0.translation.z += -h_cols + col as f32 * cell_width;
			maze_wall::spawn(
				&mut commands,
				&mut meshes,
				&mut materials,
				&wall.0,
				&wall.1,
			);
		}
	}
}
