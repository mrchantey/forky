use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;




pub fn draw_cube(
	position: Vec3,
	size: f32,
	color: Color,
	mut lines: ResMut<DebugLines>,
) {
	let half_size = size / 2.;

	let p0 = position + Vec3::new(-half_size, half_size, half_size);
	let p1 = position + Vec3::new(-half_size, -half_size, half_size);
	let p2 = position + Vec3::new(half_size, -half_size, half_size);
	let p3 = position + Vec3::new(half_size, half_size, half_size);

	let p4 = position + Vec3::new(-half_size, half_size, -half_size);
	let p5 = position + Vec3::new(-half_size, -half_size, -half_size);
	let p6 = position + Vec3::new(half_size, -half_size, -half_size);
	let p7 = position + Vec3::new(half_size, half_size, -half_size);



	//front
	lines.line_colored(p0, p1, 0.0, color);
	lines.line_colored(p1, p2, 0.0, color);
	lines.line_colored(p2, p3, 0.0, color);
	lines.line_colored(p3, p0, 0.0, color);
	//back
	lines.line_colored(p4, p5, 0.0, color);
	lines.line_colored(p5, p6, 0.0, color);
	lines.line_colored(p6, p7, 0.0, color);
	lines.line_colored(p7, p4, 0.0, color);
	//sides
	lines.line_colored(p0, p4, 0.0, color);
	lines.line_colored(p1, p5, 0.0, color);
	lines.line_colored(p2, p6, 0.0, color);
	lines.line_colored(p3, p7, 0.0, color);
}
