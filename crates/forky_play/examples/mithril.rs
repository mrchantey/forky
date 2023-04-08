use bevy::prelude::*;
use forky_play::*;
fn main() {
	App::new()
		.add_plugin(plugins::ForkyDebugPlugin::without_debug_cameras())
		.add_plugin(plugins::RotateCubePlugin)
		.add_startup_system(setup)
		.run();
}


fn setup(mut commands: Commands) {
	commands.spawn(camera::FlyCameraBundle::forward());
}
