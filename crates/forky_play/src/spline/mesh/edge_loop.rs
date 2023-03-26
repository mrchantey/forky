use bevy::prelude::*;



///wound anticlockwise from top left
pub fn rect_edge_loop(width: f32, height: f32) -> Vec<Vec2> {
	let half_width = width / 2.;
	let half_height = height / 2.;
	vec![
		Vec2::new(-half_width, half_height),
		Vec2::new(-half_width, -half_height),
		Vec2::new(half_width, -half_height),
		Vec2::new(half_width, half_height),
	]
}
