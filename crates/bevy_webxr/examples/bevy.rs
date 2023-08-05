use bevy::prelude::*;
use bevy_webxr::*;

fn main() {
	set_panic_hook();
	App::new().add_plugins(demo::DemoScenePlugin).run();
}
