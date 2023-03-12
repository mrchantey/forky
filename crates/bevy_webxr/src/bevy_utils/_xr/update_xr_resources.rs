use crate::*;
use crate::*;
use bevy::prelude::*;
use web_sys::*;
use wgpu::Extent3d;




pub fn update_xr_resources(world: &mut World) {
	let frame = world.non_send_resource::<XrFrame>();
	let gl_layer = frame.session().render_state().base_layer().unwrap();

	let reference_space = world.non_send_resource::<XrReferenceSpace>();

	//never None?
	let pose = frame.get_viewer_pose(&reference_space).unwrap();

	let views = pose
		.views()
		.iter()
		.map(|view| view.into())
		.collect::<Vec<_>>();

	let bevy_views = views
		.iter()
		.map(|view| bevy_utils::BevyXrView::new(view, &gl_layer))
		.collect::<Vec<_>>();

	world.insert_non_send_resource(gl_layer);
	world.insert_non_send_resource(pose);
	world.insert_non_send_resource(views);
	world.insert_non_send_resource(bevy_views);
}
