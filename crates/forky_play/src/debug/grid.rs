//https://gist.github.com/leonskidev/080181bfa6104ee0824547b4f819faa3
use crate::*;
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use forky_core::math::f;

pub struct GridPlugin;

impl Plugin for GridPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_startup_system(draw_grid)
			.add_startup_system(draw_grid_axis)
			.__();
	}
}

const DURATION: f32 = f32::MAX;
const Y_OFFSET: f32 = -0.01;

fn draw_grid(mut lines: ResMut<DebugLines>) {
	let cell_size = 1_f32;
	let num_cells = 10;
	let grid_width = cell_size * f(num_cells);

	let h_grid_width = grid_width / 2.;

	for i in 0..=num_cells {
		let x0 = -h_grid_width + f(i) * cell_size;
		lines.line_colored(
			Vec3::new(x0, Y_OFFSET, -h_grid_width),
			Vec3::new(x0, Y_OFFSET, h_grid_width),
			DURATION,
			Color::GRAY.with_a(0.2),
		);
		lines.line_colored(
			Vec3::new(-h_grid_width, Y_OFFSET, x0),
			Vec3::new(h_grid_width, Y_OFFSET, x0),
			DURATION,
			Color::GRAY.with_a(0.2),
		);
	}
}


fn draw_grid_axis(mut lines: ResMut<DebugLines>) {
	lines.line_colored(
		Vec3::ZERO,
		Vec3::X,
		DURATION,
		Color::RED,
	);
	lines.line_colored(
		Vec3::ZERO,
		Vec3::Y,
		DURATION,
		Color::GREEN,
	);
	lines.line_colored(
		Vec3::ZERO,
		Vec3::Z,
		DURATION,
		Color::BLUE,
	);
}
