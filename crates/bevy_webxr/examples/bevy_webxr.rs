use bevy::prelude::*;
use bevy_webxr::*;

fn main() {
	let mut app = App::new();
	app.add_plugin(demo::SimplePlugin);
	// app.add_plugin(demo::SimpleNoWinitPlugin);
	app.run_webxr();
	// .add_plugins(DefaultPlugins.build().disable::<WinitPlugin>())
	// .add_plugin(demo::SimpleHeadlessPlugin);
}
