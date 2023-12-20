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
			.add_systems(Update,utility::create_exit_after_system(2.))
			.run();

	}
}
