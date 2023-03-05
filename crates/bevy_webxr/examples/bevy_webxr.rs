use bevy::prelude::*;
use bevy_webxr::*;

fn main() {
	let mut app = App::new();
	app
		// .add_plugins(DefaultPlugins.build().disable::<WinitPlugin>())
		.add_plugin(demo::SimplePlugin);
	// .add_plugin(demo::SimpleHeadlessPlugin);
	bevy_webxr::bevy_xr_utils::run_bevy_webxr(app);
}
