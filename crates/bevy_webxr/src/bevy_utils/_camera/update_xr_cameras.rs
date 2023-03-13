use crate::*;
use bevy::{prelude::*, render::camera::Viewport};
use web_sys::*;

pub fn update_xr_cameras(
	views: NonSend<Vec<bevy_utils::BevyXrView>>,
	mut query: Query<(&bevy_utils::XrCamera, &mut Transform)>,
) {

	for (xr_camera, mut transform) in query.iter_mut() {
		let view = &views[xr_camera.index];

		transform.translation = view.pose.position;
		transform.rotation = view.pose.rotation;

	}
}
