use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::winit::WinitPlugin;
use bevy_mod_debugdump::schedule_graph::Settings;
use gamai::*;
use std::fs::File;
use std::io::Write;


#[action]
fn root() {}
#[action]
fn child1() {}
#[action]
fn child2() {}
#[action]
fn child3() {}

pub fn main() -> Result<()> {
	let tree1 = || {
		tree! {
			<root>
					<child1 apply_deferred>
						<child2 apply_deferred>
							<child3 apply_deferred/>
						</child2>
					</child1>
			</root>
		}
	};

	let mut app = App::new();

	app.add_plugins(TreePlugin::new(tree1));
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
