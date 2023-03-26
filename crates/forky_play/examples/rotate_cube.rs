use bevy::prelude::*;
use forky_play::*;
fn main() {
	App::new()
		.add_plugin(plugins::ForkyDebugPlugin)
		.add_plugin(plugins::RotateCubePlugin)
		.run();
}
