//https://gist.github.com/leonskidev/080181bfa6104ee0824547b4f819faa3
use crate::*;
use bevy::{
    color::palettes::tailwind::{BLUE_500, GRAY_500, GREEN_500, RED_500},
    prelude::*,
};
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
            GRAY_500.with_alpha(0.5),
        );
        gizmos.line(
            Vec3::new(-h_grid_width, Y_OFFSET, x0),
            Vec3::new(h_grid_width, Y_OFFSET, x0),
            GRAY_500.with_alpha(0.5),
        );
    }
}

fn draw_grid_axis(mut gizmos: Gizmos) {
    gizmos.line(Vec3::ZERO, Vec3::X, RED_500);
    gizmos.line(Vec3::ZERO, Vec3::Y, GREEN_500);
    gizmos.line(Vec3::ZERO, Vec3::Z, BLUE_500);
}
