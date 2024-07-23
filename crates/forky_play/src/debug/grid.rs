//https://gist.github.com/leonskidev/080181bfa6104ee0824547b4f819faa3
use crate::*;
use bevy::prelude::*;
use forky_core::math::f;

pub struct GridPlugin;

impl Plugin for GridPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_systems(Update, draw_grid)
			.add_systems(Update, draw_grid_axis)
			.__();
	}
}

const Y_OFFSET: f32 = -0.01;

fn draw_grid(mut gizmos: Gizmos) {
	let cell_size = 1_f32;
	let num_cells = 10;
	let grid_width = cell_size * f(num_cells);

	let h_grid_width = grid_width / 2.;

	for i in 0..=num_cells {
		let x0 = -h_grid_width + f(i) * cell_size;
		gizmos.line(
			Vec3::new(x0, Y_OFFSET, -h_grid_width),
			Vec3::new(x0, Y_OFFSET, h_grid_width),
			Color::GRAY.with_a(0.5),
		);
		gizmos.line(
			Vec3::new(-h_grid_width, Y_OFFSET, x0),
			Vec3::new(h_grid_width, Y_OFFSET, x0),
			Color::GRAY.with_a(0.5),
		);
	}
}


fn draw_grid_axis(mut gizmos: Gizmos) {
	gizmos.line(Vec3::ZERO, Vec3::X, Color::RED);
	gizmos.line(Vec3::ZERO, Vec3::Y, Color::GREEN);
	gizmos.line(Vec3::ZERO, Vec3::Z, Color::BLUE);
}
