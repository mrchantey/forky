use bevy::prelude::*;
use forky_play::*;
use sweet::*;
// use crate::maze::plugin::MazePlugin;
sweet! {

	it skip "works" {
		App::new()
			.add_plugin(plugins::ForkyPlugin)
			.add_plugin(maze::MazePlugin)
			.add_startup_system(utility::surrender_focus)
			.forky_exit_after(2.)
			.run();

	}
}
