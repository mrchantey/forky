use bevy::prelude::*;
use forky_play::*;
fn main() {
	App::new()
		// .add_plugins(DefaultPlugins)
		.add_plugin(plugins::CustomDefaultPlugin)
		.add_plugin(plugins::SimplePlugin)
		.run();
}
