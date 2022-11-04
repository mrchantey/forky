use bevy::prelude::*;
use forky_core::{math::*, *};
use forky_play::{maze::*, *};
use sweet::*;
// use crate::maze::plugin::MazePlugin;
sweet! {

	it skip "works" {
		app::init()
			// .add_plugin(maze::MazePlugin)
			.add_startup_system(utility::surrender_focus)
			.add_startup_system(maze_3d::spawn)
			// .forky_exit_after(2)
			.run();
	}
}
