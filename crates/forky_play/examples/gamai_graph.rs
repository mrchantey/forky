#![feature(return_position_impl_trait_in_trait)]
// use bevy_app::*;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::winit::WinitPlugin;
use bevy_mod_debugdump::schedule_graph::Settings;
use gamai::*;
use std::fs::File;
use std::io::Write;


#[node_system]
fn root() {}
#[node_system]
fn node1() {}
// fn edge1() {}
#[node_system]
fn node2() {}
#[node_system]
fn edge2() {}
#[node_system]
fn node3() {}
#[node_system]
fn edge3() {}

pub fn main() -> Result<()> {
	let tree1 = tree! {
		<root>
				// <node1>
					// <node2 edge=edge2>
						// <node3 edge=edge3/>
					// </node2>
					// <node_always_succeed>
					// 	<node_always_succeed>
					// 		<node_always_succeed/>
					// 	</node_always_succeed>
					// </node_always_succeed>
				// </node1>
		</root>
	};
	// let tree2 = tree! {
	// 			<node_always_succeed>
	// 				<node_always_succeed/>
	// 				<node_always_succeed/>
	// 			</node_always_succeed>
	// };

	let mut app = App::new();

	app.add_plugins(tree1.plugin());
	// app.add_plugins(tree2.plugin());
	app.add_plugins(
		DefaultPlugins
			.build()
			.disable::<LogPlugin>()
			.disable::<WinitPlugin>(),
	);
	let render_graph = bevy_mod_debugdump::schedule_graph_dot(
		&mut app,
		Update,
		&Settings::default(),
	);
	let path = "target/graph";
	std::fs::create_dir_all(path)?;
	let mut file = File::create("target/graph/render_graph.dot").unwrap();
	file.write_all(render_graph.as_bytes()).unwrap();
	Ok(())
}
