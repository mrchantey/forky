use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::winit::WinitPlugin;
use bevy_mod_debugdump::schedule_graph::Settings;
// use gamai::*;
use std::fs::File;
use std::io::Write;
#[derive(Debug, Default, SystemSet, Clone, Eq, PartialEq, Hash)]
pub struct Set1;
#[derive(Debug, Default, SystemSet, Clone, Eq, PartialEq, Hash)]
pub struct Set2(pub usize);


pub fn main() -> anyhow::Result<()> {
	let mut app = App::new();

	app.configure_set(Update, Set1.before(Set2(1)));
	app.configure_set(Update, Set1.before(Set2(7)));
	// app.add_plugins(tree2.plugin());
	app.add_plugins(
		DefaultPlugins
			.build()
			.disable::<LogPlugin>()
			.disable::<WinitPlugin>(),
	);
	let render_graph =
		bevy_mod_debugdump::schedule_graph_dot(&mut app, Update, &Settings {
			// include_system: Some(Box::new(|_| false)),
			..default()
		});
	let path = "target/graph";
	std::fs::create_dir_all(path)?;
	let mut file = File::create("target/graph/render_graph.dot").unwrap();
	file.write_all(render_graph.as_bytes()).unwrap();
	Ok(())
}
