use bevy::prelude::*;
use web_sys::*;

pub fn insert_gl_layer(world: &mut World) {
	let gl_layer = world
		.non_send_resource::<XrFrame>()
		.session()
		.render_state()
		.base_layer()
		.unwrap();
	world.insert_non_send_resource(gl_layer);
}
