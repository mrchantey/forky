use crate::*;
use bevy::prelude::*;
use forky_core::math::*;
use super::*;

pub struct MazePlugin;


impl Plugin for MazePlugin {
	fn build(&self, app: &mut App) {
		app.forky()
		.add_startup_system(maze_3d::spawn)
		.add_system(board::controller)
			.forky();
	}
}