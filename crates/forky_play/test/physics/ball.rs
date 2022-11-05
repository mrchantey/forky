use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use forky_core::*;
use forky_play::maze::{ball, board, floor, RectMazeSpatial};
use forky_play::OptI32X;
use forky_play::*;
use sweet::*;

sweet! {
	it "works" {
		app::init()
		// .forky_exit_after(5)
				.add_startup_system(utility::surrender_focus)
				.add_startup_system(spawn)
				// .add_system(board::force_controller)
				.run();
	}
}

pub fn spawn(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let cell_width = 1.;

	let maze = RectMazeSpatial::default();
	board::spawn(&mut commands, &mut meshes, &mut materials, &maze);
	ball::spawn(&mut commands, &mut meshes, &mut materials, &maze);
}
