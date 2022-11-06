use super::*;
use crate::{OptI32X, *};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use forky_core::math::*;
use forky_core::*;
pub struct MazePlugin;


impl Plugin for MazePlugin {
	fn build(&self, app: &mut App) {
		app.forky()
			.insert_resource(board_joint::MazeJointParams::default())
			.add_event::<maze_3d::RespawnEvent>()
			.add_event::<maze_3d::DespawnEvent>()
			.add_system(maze_3d::respawn)
			.add_system(maze_3d::despawn)
			.add_system(board::respawn)
			.insert_resource(RapierConfiguration::with_gravity_scalar(10.))
			.add_system(ball::respawn)
			.add_system(ball::despawn_on_ball_fall)
			.add_system(board_joint::force_controller)
			.forky();
	}
}
