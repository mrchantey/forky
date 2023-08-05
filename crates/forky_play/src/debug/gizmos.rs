use bevy::prelude::*;

pub fn draw_cube(position: Vec3, size: f32, color: Color, mut gizmos: Gizmos) {
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
	gizmos.line(p0, p1, color);
	gizmos.line(p1, p2, color);
	gizmos.line(p2, p3, color);
	gizmos.line(p3, p0, color);
	//back
	gizmos.line(p4, p5, color);
	gizmos.line(p5, p6, color);
	gizmos.line(p6, p7, color);
	gizmos.line(p7, p4, color);
	//sides
	gizmos.line(p0, p4, color);
	gizmos.line(p1, p5, color);
	gizmos.line(p2, p6, color);
	gizmos.line(p3, p7, color);
}
