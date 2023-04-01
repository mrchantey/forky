use bevy::prelude::*;
use bevy_mod_debugdump::get_render_graph;
use forky_play::*;
use std::fs::File;
use std::io::Write;

fn main() {
	let mut app = App::new();
	app.add_plugins(DefaultPlugins)
		.add_plugin(render_graph::ClearGraphPlugin);
	let render_graph = get_render_graph(&mut app);
	let mut file = File::create("target/render_graph.dot").unwrap();
	file.write_all(render_graph.as_bytes()).unwrap();
}
