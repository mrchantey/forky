use crate::*;
use bevy::{prelude::*, render::camera::Viewport};
use web_sys::*;
use wgpu::Extent3d;

pub fn update_xr_cameras(
	// frame: NonSend<XrFrame>,
	views: NonSend<Vec<bevy_utils::BevyXrView>>,
	reference_space: NonSend<XrReferenceSpace>,
	mut query: Query<(&bevy_utils::XrCamera, &mut Transform)>,
) {
	// let gl_layer = frame.session().render_state().base_layer().unwrap();

	for (xr_camera, mut transform) in query.iter_mut() {
		let view = &views[xr_camera.index];

		//transform
		// let (translation, rotation) = bevy_utils::view_pose(&view);
		transform.translation = view.position;
		transform.rotation = view.rotation;

	}
}
