use anyhow::*;
use bevy::prelude::*;
use forky_play::spline::graph::{on_node_moved, SplineGraph};
use forky_play::spline::*;
use forky_play::*;
use sweet::*;
sweet! {
	test "node handles" {
		let mut app = App::new();
		app.__()
			.add_system(on_node_moved)
			.__();

		let entity = app.world.spawn(spline::graph::SplineNodeBundle::new(
			Vec3::ZERO,
			spline::graph::SplineNode(0),
		)).id();


		app.update();
		app.world.entity_mut(entity)
		.insert(Transform::from_translation(Vec3::new(1., 0., 0.)));
		app.update();

	}


}
