use super::types::*;
use super::*;
use crate::{OptI32X, *};
use bevy::prelude::*;
use bevy_rapier3d::prelude::RapierConfiguration;
// use bevy_rapier3d::prelude::*;
pub struct MazePlugin;


impl Plugin for MazePlugin {
	fn build(&self, app: &mut App) {
		app.forky()
			.add_event::<RespawnEvent>()
			.add_event::<DespawnEvent>()
			.insert_resource(MazeGame::default())
			.insert_resource(board_joint::MazeJointParams::default())
			.add_system(systems::increment_score)
			.add_system(maze_3d::respawn)
			.add_system(maze_3d::despawn)
			.add_system(board::respawn)
			.add_system(systems::adjust_camera_on_respawn)
			.add_system(systems::reset_game_on_key)
			.insert_resource(RapierConfiguration::with_gravity_scalar(10.))
			.add_system(ball::respawn)
			.add_system(ball::despawn_on_ball_fall)
			.add_system(board_joint::force_controller)
			.add_startup_system(ui::spawn)
			.add_system(ui::update)
			.forky();
	}
}
