use bevy::prelude::*;
use forky_play::*;

fn main() {
	App::new()
		.add_plugin(plugins::CustomDefaultPlugin)
		.add_plugin(plugins::SimplePlugin)
		// .add_plugin(graph::BlitGraphPlugin)
		// .add_plugin(graph::ClearGraphPlugin)
		.add_plugin(graph::CustomPipelinePlugin)
		.add_system(utility::create_exit_after_system(4.))
		.run();
}
