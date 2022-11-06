use super::*;
use crate::*;
use bevy::prelude::*;
use forky_core::math::*;
pub struct MazePlugin;


impl Plugin for MazePlugin {
	fn build(&self, app: &mut App) {
		app.forky()
			.insert_resource(board_joint::MazeJointParams::default())
			.add_startup_system(maze_3d::spawn)
			.add_system(board_joint::force_controller)
			.forky();
	}
}
