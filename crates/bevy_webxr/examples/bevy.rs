use bevy::prelude::*;
use bevy_webxr::*;

fn main() {
	set_panic_hook();
	App::new().add_plugin(demo::DemoScenePlugin).run();
}
