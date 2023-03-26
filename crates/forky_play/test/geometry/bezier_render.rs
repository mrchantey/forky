use bevy::prelude::*;
use forky_play::*;
use sweet::*;


sweet! {

	it skip "works" {
		let mut app = App::new();
		app.__()
			.forky_exit_after(3.)
			.add_plugin(plugins::ForkyDebugPlugin)
			.__()
			.run();
	}
}
