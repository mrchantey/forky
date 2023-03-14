use crate::*;
use bevy::prelude::*;
use web_sys::*;
use wgpu::Extent3d;

use super::{BevyXrView, RawProjection, BevyXrViewLookup};




pub fn cameras_need_rebuild(
	query: Query<(&bevy_utils::XrCamera, &BevyXrView)>,
	gl_layer: NonSend<web_sys::XrWebGlLayer>,
	views: Res<BevyXrViewLookup>,
) -> bool {
	if query.iter().count() != views.len() {
		return true;
	}

	for (xr_camera, cam_view) in query.iter() {
		let view = &views[xr_camera.index];
		if view.hash != cam_view.hash {
			return true;
		}
	}
	false
}
