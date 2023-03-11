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
	frame: NonSend<XrFrame>,
	reference_space: NonSend<XrReferenceSpace>,
	mut query: Query<(&mut Transform, &mut Camera, &bevy_utils::XrCamera)>,
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

	for (mut transform, mut camera, xr_camera) in query.iter_mut() {
		let view = &views[xr_camera.index];
		let translation =
			bevy_utils::dom_point_to_vec3(&view.transform().position());
		let rotation =
			bevy_utils::dom_point_to_quat(&view.transform().orientation());
		transform.translation = translation;
		transform.rotation = rotation;

		// TODO requires rendertarget::textureview
		// let viewport = gl_layer.get_viewport(view).unwrap();
		// let viewport = Viewport {
		// 	physical_position: UVec2::new(
		// 		viewport.x() as u32,
		// 		viewport.y() as u32,
		// 	),
		// 	physical_size: UVec2::new(
		// 		viewport.width() as u32,
		// 		viewport.height() as u32,
		// 	),
		// 	..default()
		// };
		// camera.viewport = Some(viewport);
	}
}
