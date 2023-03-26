use bevy::prelude::*;
use forky_play::{maze::MazePlugin, *};

fn main() {
	App::new()
		.add_plugin(plugins::ForkyFullPlugin)
		.add_plugin(MazePlugin)
		// .forky_exit_after(2)
		.run();
}
