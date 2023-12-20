use bevy::prelude::*;
use forky_play::maze::MazePlugin;
use forky_play::*;

fn main() {
	App::new()
		.add_plugins(plugins::ForkyFullPlugin::default())
		.add_plugins(MazePlugin)
		// .add_systems(Update,utility::create_exit_after_system(3.))
		.run();
}
