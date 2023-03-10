use crate::*;
use bevy::{prelude::*, render::camera::Viewport};
use web_sys::*;
use wgpu::Extent3d;



// #[derive(Component)]
// pub struct XrReferenceFrame{
// 	reference_frame: XrReferenceSpace,
// }
// struct Viewport {
// 	x: f32,
// 	y: f32,
// 	width: f32,
// 	height: f32,
// }



pub fn update_xr_tracking(
	app: &mut App,
	frame: &XrFrame,
	reference_space: &XrReferenceSpace,
) {
	let gl_layer = frame.session().render_state().base_layer().unwrap();

	let width = gl_layer.framebuffer_width();
	let height = gl_layer.framebuffer_width();


	let pose = match frame.get_viewer_pose(&reference_space) {
		Some(pose) => pose,
		None => return,
	};

	let views: Vec<web_sys::XrView> =
		pose.views().iter().map(|view| view.into()).collect();

	let transforms: Vec<_> = views
		.iter()
		.map(|view| {
			(
				bevy_utils::dom_point_to_vec3(&view.transform().position()),
				bevy_utils::dom_point_to_quat(&view.transform().orientation()),
			)
		})
		.collect();


	let viewports: Vec<_> = views
		.iter()
		.map(|view| {
			let viewport = gl_layer.get_viewport(view).unwrap();

			Viewport {
				physical_position: UVec2::new(
					viewport.x() as u32,
					viewport.y() as u32,
				),
				physical_size: UVec2::new(
					viewport.width() as u32,
					viewport.height() as u32,
				),
				..default()
			}
		})
		.collect();

	let mut query =
		app.world
			.query::<(&mut Transform, &mut Camera, With<bevy_utils::XrCamera>)>(
			);

	for (i, (mut transform, mut camera, _)) in
		query.iter_mut(&mut app.world).enumerate()
	{
		transform.translation = transforms[i].0;
		transform.rotation = transforms[i].1;
		// camera.viewport = Some(viewports[i].clone());
	}
}
