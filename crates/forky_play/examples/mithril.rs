use bevy::prelude::*;
use forky_play::{
	spline::{ecs_graph, Spline},
	*,
};
fn main() {
	let height = 3.;
	let width = 3.;

	App::new()
		.add_plugin(plugins::ForkyFullPlugin {
			debug_plugin: plugins::ForkyDebugPlugin {
				debug_cameras: false,
				debug_grid: false,
			},
			tool_plugin: tool::ToolPlugin {
				settings: tool::InteractionSettings {
					intersect_normal: Vec3::Z_NEG,
					..default()
				},
			},
			..default()
		})
		.add_plugin(spline::SplinePlugin)
		.add_startup_system(setup)
		.add_startup_system(ecs_graph::create_graph_with_spline(
			Spline::Cubic(spline::CubicSpline::new(
				Vec3::new(-width, height, 0.),
				Vec3::new(-width + 1., height, 0.),
				Vec3::new(width - 1., height, 0.),
				Vec3::new(width, height, 0.),
			)),
			false,
		))
		.run();
}


fn setup(mut commands: Commands) {
	commands.spawn(camera::FlyCameraBundle::forward());
}
