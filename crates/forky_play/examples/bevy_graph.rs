use bevy::prelude::*;
use forky_play::*;

fn main() {
	App::new()
		.add_plugin(plugins::ForkyDebugPlugin)
		.add_plugin(plugins::RotateCubePlugin)
		// .add_plugin(graph::BlitGraphPlugin)
		// .add_plugin(graph::ClearGraphPlugin)
		.add_plugin(render_graph::CustomPipelinePlugin)
		.add_system(utility::create_exit_after_system(4.))
		.run();
}
