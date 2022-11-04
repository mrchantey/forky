use bevy::prelude::*;
use forky_core::{math::*, *};
use forky_play::*;
use sweet::*;
// use crate::maze::plugin::MazePlugin;
sweet! {

	it "works" {
		app::init()
			.add_plugin(maze::MazePlugin)
			.add_plugin(input::DebugCameraPlugin)
			// .forky_exit_after(2)
			.run();
	}
}

