use bevy::prelude::*;
use forky_play::maze::MazePlugin;
use forky_play::*;

fn main() {
	App::new()
		.add_plugins(plugins::ForkyFullPlugin::default())
		.add_plugins(MazePlugin)
		// .forky_exit_after(2)
		.run();
}
