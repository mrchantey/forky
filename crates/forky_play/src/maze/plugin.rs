use super::types::*;
use super::*;
use crate::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::RapierConfiguration;
// use bevy_rapier3d::prelude::*;
pub struct MazePlugin;


impl Plugin for MazePlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_event::<RespawnEvent>()
			.add_event::<DespawnEvent>()
			.insert_resource(MazeGame::default())
			.insert_resource(board_joint::MazeJointParams::default())
			.add_systems(Update, systems::increment_score)
			.add_systems(Update, maze_3d::respawn)
			.add_systems(Update, maze_3d::despawn)
			.add_systems(Update, board::respawn)
			.add_systems(Update, systems::adjust_camera_on_respawn)
			.add_systems(Update, systems::reset_game_on_key)
			.insert_resource(RapierConfiguration::with_gravity_scalar(10.))
			.add_systems(Update, ball::respawn)
			.add_systems(Update, ball::despawn_on_ball_fall)
			.add_systems(Update, board_joint::force_controller)
			.add_systems(Startup, ui::spawn)
			.add_systems(Startup, spawn_lights)
			.add_systems(Update, ui::update)
			.__();
	}
}

fn spawn_lights(mut commands: Commands) {
	commands.spawn(PointLightBundle {
		transform: Transform::from_xyz(-5., 5., 3.),
		point_light: PointLight {
			intensity: 1000.,
			color: Color::FUCHSIA,
			shadows_enabled: true,
			..default()
		},
		..default()
	});
	commands.spawn(PointLightBundle {
		transform: Transform::from_xyz(3., 5., -5.),
		point_light: PointLight {
			intensity: 1000.,
			shadows_enabled: true,
			color: Color::CYAN,
			..default()
		},
		..default()
	});
}
