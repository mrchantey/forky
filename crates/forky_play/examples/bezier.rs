use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use forky_play::*;


fn main() {
	App::new()
		.add_plugin(plugins::CustomDefaultPlugin)
		// .add_plugin(plugins::SimplePlugin)
		.add_startup_system(plugins::spawn_default_camera)
		.add_plugin(DebugLinesPlugin::with_depth_test(true))
		.add_system(draw_cubic)
		.run();
}



pub fn draw_cubic(mut lines: ResMut<DebugLines>) {
	let p0 = Vec3::new(-1., 1., 0.);
	let p1 = Vec3::new(-1., -1., 0.);
	let p2 = Vec3::new(1., -1., 0.);
	let p3 = Vec3::new(1., 1., 0.);

	let len = 10;
	let path = forky_play::path::bezier_path_3d(p0, p1, p2, p3, len);

	for i in 0..len {
		let i = i as usize;
		lines.line_colored(path[i], path[i + 1], 0.0, Color::GRAY.with_a(0.2));
	}
}
