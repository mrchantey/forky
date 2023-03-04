use bevy::prelude::*;
use bevy_webxr::*;

fn main() {
	let mut app = App::new();
	app
		// .add_plugins(DefaultPlugins.build().disable::<WinitPlugin>())
		.add_plugin(demo::SimplePlugin);
	bevy_webxr::app::run_bevy_webxr(app);
}
