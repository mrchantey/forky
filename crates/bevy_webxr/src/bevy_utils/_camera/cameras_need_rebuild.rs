use crate::*;
use bevy::prelude::*;
use web_sys::*;
use wgpu::Extent3d;




pub fn cameras_need_rebuild(
	query: Query<(&bevy_utils::XrCamera,&Camera, &PerspectiveProjection)>,
	frame: NonSend<web_sys::XrFrame>,
	views: NonSend<Vec<bevy_utils::BevyXrView>>,
) -> bool {
	if query.iter().count() == 0 {
		return true;
	}

	for (xr_camera,camera,projection) in query.iter() {
		let view = &views[xr_camera.index];
		if view.needs_rebuild(&camera,&projection) {
			return true;
		}
	}
	false
}
