use bevy::prelude::*;
use forky_play::*;
fn main() {
	App::new()
		.add_plugins(plugins::ForkyDebugPlugin::default())
		.add_plugins(plugins::RotateCubePlugin)
		.run();
}
