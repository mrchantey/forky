use bevy::prelude::*;
use forky_play::*;

fn main() {
	App::new()
		.add_plugins(plugins::ForkyDebugPlugin::default())
		.add_plugins(plugins::RotateCubePlugin)
		// .add_plugins(graph::BlitGraphPlugin)
		// .add_plugins(graph::ClearGraphPlugin)
		.add_plugins(render_graph::CustomPipelinePlugin)
		.add_systems(Update, utility::create_exit_after_system(4.))
		.run();
}
