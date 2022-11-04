//https://gist.github.com/leonskidev/080181bfa6104ee0824547b4f819faa3
use crate::*;
use bevy::prelude::*;
use forky_core::math::f;

use bevy_inspector_egui::Inspectable;
use bevy_prototype_debug_lines::*;

pub struct GridPlugin;

impl Plugin for GridPlugin {
	fn build(&self, app: &mut App) {
		app.forky()
			.add_plugin(DebugLinesPlugin::with_depth_test(true))
			.add_system(draw_grid)
			// .add_system(some_system)
			.add_system(draw_grid_axis)
			.add_startup_system(spawn_grid)
			.forky();
	}
}

fn spawn_grid(mut commands: Commands) {
	commands.spawn_bundle(SpatialBundle::default()).insert(Grid);
}


#[derive(Debug, Component)]
pub struct Grid;

// fn some_system(
// 	//  ...
// 	mut lines: ResMut<DebugLines>,
// ) {
// 	let start = Vec3::splat(-10.0);
// 	let end = Vec3::splat(10.0);
// 	let duration = 0.0; // Duration of 0 will show the line for 1 frame.
// 	lines.line(start, end, duration);
// }


fn draw_grid(query: Query<&Transform, With<Grid>>, mut lines: ResMut<DebugLines>) {
	for transform in query.iter() {
		let cell_size = 1_f32;
		let num_cells = 10;
		let grid_width = cell_size * f(num_cells);

		let h_grid_width = grid_width / 2.;

		for i in 0..=num_cells {
			// let i = i as f32;
			let x0 = -h_grid_width + f(i) * cell_size;
			lines.line_colored(
				Vec3::new(x0, 0., -h_grid_width),
				Vec3::new(x0, 0., h_grid_width),
				0.0,
				Color::GRAY.with_a(0.2),
			);
			lines.line_colored(
				Vec3::new(-h_grid_width, 0., x0),
				Vec3::new(h_grid_width, 0., x0),
				0.0,
				Color::GRAY.with_a(0.2),
				// Color::GRAY,
			);
		}
	}
}

fn draw_grid_axis(query: Query<&Transform, With<Grid>>, mut lines: ResMut<DebugLines>) {
	for transform in query.iter() {
		lines.line_colored(Vec3::ZERO.add_y(0.1), Vec3::X.add_y(0.1), 0., Color::RED);
		lines.line_colored(Vec3::ZERO.add_y(0.1), Vec3::Y.add_y(0.1), 0., Color::GREEN);
		lines.line_colored(Vec3::ZERO.add_y(0.1), Vec3::Z.add_y(0.1), 0., Color::BLUE);
	}
}
