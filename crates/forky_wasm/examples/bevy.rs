// #![cfg(web_sys_unstable_apis)]
use bevy::{prelude::*, winit::*};
use forky_play::{app::SimplePlugin, *};
use forky_wasm::*;

fn main() {
	core::set_panic_hook();
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugin(SimplePlugin)
		.run();
}
