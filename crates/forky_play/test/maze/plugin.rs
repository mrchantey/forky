use bevy::prelude::*;
use forky_play::*;
use sweet::*;
// use crate::maze::plugin::MazePlugin;
sweet! {

	it skip "works" {
		App::new()
			.add_plugins(plugins::ForkyFullPlugin::default())
			.add_plugins(maze::MazePlugin)
			.add_systems(Startup, utility::surrender_focus)
			.forky_exit_after(2.)
			.run();

	}
}
