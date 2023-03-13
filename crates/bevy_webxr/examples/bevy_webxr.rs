use bevy::prelude::*;
use bevy_webxr::*;

fn main() {
	let mut app = App::new();
	app.__()
		.add_plugin(demo::DemoScenePlugin)
		.add_plugin(bevy_utils::WebXrPlugin {
			// session_mode: web_sys::XrSessionMode::Inline,
			..default()
		})
		.__();
	app.run_webxr();
}
