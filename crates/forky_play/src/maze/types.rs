use bevy::prelude::*;



#[derive(Resource, Default)]
pub struct MazeGame {
	pub score: usize,
}


pub struct RespawnEvent {
	pub level: usize,
	pub num_cols: usize,
	pub num_rows: usize,
	pub cell_width: f32,
	pub wall_width: f32,
	pub wall_height: f32,
}

#[derive(PartialEq)]
pub enum DespawnEvent {
	Success,
	Reset,
}

#[derive(Component)]
pub struct MazeItemTag;

impl Default for RespawnEvent {
	fn default() -> Self {
		Self {
			level: 1,
			num_cols: 2,
			num_rows: 2,
			cell_width: 2.,
			wall_width: 0.2,
			wall_height: 0.5,
		}
	}
}
