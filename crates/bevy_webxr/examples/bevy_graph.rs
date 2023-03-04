// #![cfg(web_sys_unstable_apis)]
use bevy::prelude::*;
// use bevy_webxr::*;

fn main() {
	// core::set_panic_hook();
	App::new().add_plugin(bevy_webxr::demo::SimplePlugin).run();
}
