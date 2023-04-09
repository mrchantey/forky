use crate::spline::ecs_graph::EcsSplineGraphLookup;
use crate::spline::graph::*;
use crate::spline::*;
use crate::*;
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
// use forky_play::spline::physics;


pub fn draw_spline(
	mut lines: ResMut<DebugLines>,
	query: Query<&Spline, Without<SplineGraphId>>,
) {
	for spline in &query {
		draw(&mut lines, spline);
	}
}

pub fn draw_graph(
	mut lines: ResMut<DebugLines>,
	graphs: Res<SplineGraphLookup>,
) {
	for graph in graphs.values() {
		for edge in graph.all_edges() {
			draw(&mut lines, &edge.2.spline);
		}
	}
}
pub fn draw_ecs_graph(
	mut lines: ResMut<DebugLines>,
	graphs: Res<EcsSplineGraphLookup>,
) {
	for graph in graphs.values() {
		for edge in graph.graph.all_edges() {
			draw(&mut lines, &edge.2.spline);
		}
	}
}

fn draw(lines: &mut DebugLines, spline: &Spline) {
	let num_nodes = 10;
	let path = spline.path(num_nodes);

	for i in 0..path.len() - 1 {
		let i = i as usize;
		lines.line_colored(path[i], path[i + 1], 0.0, Color::WHITE.with_a(0.8));
	}

	let step = 1.0 / num_nodes as f32;

	let len = 0.4;

	for i in 0..path.len() {
		let t = i as f32 * step;
		let d1 = spline.derivative(t);
		let d2 = spline.derivative2(t);
		// let d3 = spline.derivative3(t);
		lines.line_colored(
			path[i],
			path[i] + d1 * len * 0.1,
			0.0,
			Color::MAROON.with_a(0.8),
		);
		lines.line_colored(
			path[i],
			path[i] + d2 * len * 0.1,
			0.0,
			Color::YELLOW.with_a(0.8),
		);
		// lines.line_colored(
		// 	path[i],
		// 	path[i] + d3 * len,
		// 	0.0,
		// 	Color::CYAN.with_a(0.8),
		// );



		let tangent = spline.tangent(t);
		let normal = spline.normal_up(t, Vec3::UP);
		let bitangent = tangent.cross(normal).normalize();
		lines.line_colored(
			path[i],
			path[i] + bitangent * len,
			0.0,
			Color::RED.with_a(0.8),
		);
		lines.line_colored(
			path[i],
			path[i] + normal * len,
			0.0,
			Color::GREEN.with_a(0.8),
		);
		lines.line_colored(
			path[i],
			path[i] + tangent * len,
			0.0,
			Color::BLUE.with_a(0.8),
		);
	}
}
