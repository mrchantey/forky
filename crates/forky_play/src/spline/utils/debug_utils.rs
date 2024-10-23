use crate::spline::graph::*;
use crate::spline::*;
use crate::*;
use bevy::color::palettes::tailwind::{BLUE_500, FUCHSIA_500, GREEN_500, RED_500, YELLOW_500};
use bevy::prelude::*;
// use forky_play::spline::physics;

pub fn draw_spline(mut gizmos: Gizmos, query: Query<&Spline, Without<SplineGraphId>>) {
    for spline in &query {
        draw(&mut gizmos, spline);
    }
}

pub fn draw_graph(mut gizmos: Gizmos, graphs: Res<SplineGraphLookup>) {
    for graph in graphs.values() {
        for edge in graph.all_edges() {
            draw(&mut gizmos, &edge.2.spline);
        }
    }
}

fn draw(gizmos: &mut Gizmos, spline: &Spline) {
    let num_nodes = 10;
    let path = spline.path(num_nodes);

    for i in 0..path.len() - 1 {
        let i = i as usize;
        gizmos.line(path[i], path[i + 1], Srgba::WHITE.with_alpha(0.8));
    }

    let step = 1.0 / num_nodes as f32;

    let len = 0.4;

    for i in 0..path.len() {
        let t = i as f32 * step;
        let d1 = spline.derivative(t);
        let d2 = spline.derivative2(t);
        // let d3 = spline.derivative3(t);
        gizmos.line(
            path[i],
            path[i] + d1 * len * 0.1,
            FUCHSIA_500.with_alpha(0.8),
        );
        gizmos.line(
            path[i],
            path[i] + d2 * len * 0.1,
            YELLOW_500.with_alpha(0.8),
        );
        // gizmos.line(
        // 	path[i],
        // 	path[i] + d3 * len,
        // 	0.0,
        // 	Color::CYAN.with_a(0.8),
        // );
        let tangent = spline.tangent(t);
        let normal = spline.normal_up(t, Vec3::UP);
        let bitangent = tangent.cross(normal).normalize();
        gizmos.line(path[i], path[i] + bitangent * len, RED_500.with_alpha(0.8));
        gizmos.line(path[i], path[i] + normal * len, GREEN_500.with_alpha(0.8));
        gizmos.line(path[i], path[i] + tangent * len, BLUE_500.with_alpha(0.8));
    }
}
