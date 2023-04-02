use bevy::prelude::*;
use forky_play::*;

fn main() {
	let mut app = App::new();
	app.__()
		.add_plugin(plugins::ForkyFullPlugin)
		.add_plugin(spline::SplinePlugin)
		.run();
}
