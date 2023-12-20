use bevy::prelude::*;
use forky_play::plugins::*;
use forky_play::*;
use sweet::*;


sweet! {

	it skip "works" {
		let mut app = App::new();
		app.__()
		// .add_systems(Update,utility::create_exit_after_system(3.))
			.add_plugins(ForkyDebugPlugin::default())
			.__()
			.run();
	}
}
