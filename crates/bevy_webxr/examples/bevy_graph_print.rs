use bevy::prelude::*;
use bevy_mod_debugdump::render_graph_dot;
use std::fs::File;
use std::io::Write;

fn main() {
	let mut app = App::new();
	app.add_plugins(DefaultPlugins);
	// .add_plugin(graph::ClearGraphPlugin);
	let render_graph = render_graph_dot(
		&mut app,
		&bevy_mod_debugdump::render_graph::Settings::default(),
	);
	let mut file = File::create("target/render_graph.dot").unwrap();
	file.write_all(render_graph.as_bytes()).unwrap();
}
