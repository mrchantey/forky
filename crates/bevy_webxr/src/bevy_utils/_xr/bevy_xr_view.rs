use crate::*;
use bevy::{prelude::*, render::camera::Viewport};
use web_sys::*;


pub struct BevyXrView {
	pub pose: bevy_utils::Pose,
	pub viewport: Viewport,
	pub projection: PerspectiveProjection,
	// pub projection: Vec<f32>,
	// pub
}


impl BevyXrView {
	pub fn new(view: &XrView, gl_layer: &XrWebGlLayer) -> Self {
		let pose: bevy_utils::Pose = view.transform().into();
		let viewport = gl_layer.get_viewport(view).unwrap();
		let viewport = bevy_utils::view_viewport(&viewport);
		let projection = view.projection_matrix();
		let mut projection = projection_from_vec(&projection);
		//fov seems to always be negative, but no issue
		// projection.fov *= -1.;
		projection.near = 0.01;
		projection.far = 1000.0;
		//otherwise bevy inverts the ar? ie 2 becomes 0.5
		projection.aspect_ratio = 1. / projection.aspect_ratio;

		Self {
			pose,
			viewport,
			projection,
		}
	}

	pub fn needs_rebuild(
		&self,
		camera: &Camera,
		projection: &PerspectiveProjection,
	) -> bool {
		if !self.projection.is_equal(projection) {
			log!(
				"rebuild for projection, current: {:?} next: {:?}",
				projection,
				self.projection,
			);
			return true;
		}
		let cam_viewport = camera.viewport.clone().unwrap();
		if !self.viewport.is_equal(&cam_viewport) {
			log!(
				"rebuild for viewport, current: {:?} next: {:?}",
				&cam_viewport,
				self.viewport,
			);
			return true;
		}
		false
	}
}
