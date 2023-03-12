use crate::*;
use bevy::{prelude::*, render::camera::Viewport};
use web_sys::*;


pub struct BevyXrView {
	pub position: Vec3,
	pub rotation: Quat,
	pub viewport: Viewport,
	pub projection: PerspectiveProjection,
	// pub projection: Vec<f32>,
	// pub
}


impl BevyXrView {
	pub fn new(view: &XrView, gl_layer: &XrWebGlLayer) -> Self {
		let (position, rotation) = bevy_utils::view_pose(&view);
		let viewport = gl_layer.get_viewport(view).unwrap();
		let viewport = bevy_utils::view_viewport(&viewport);
		let projection = view.projection_matrix();
		let mut projection = projection_from_vec(&projection);
		projection.fov *= -1.;
		projection.near = 0.01;
		projection.far = 1000.0;

		Self {
			position,
			rotation,
			viewport,
			projection,
		}
	}

	pub fn needs_rebuild(
		&self,
		camera: &Camera,
		projection: &PerspectiveProjection,
	) -> bool {
		// let (fov, ar) = crate::projection_from_vec(&self.projection);
		// let cam_projection = projection_from_mat(&cam_proj);

		//TODO camera projection looks wrong
		if !self.projection.is_equal(projection) {
			log!(
				"rebuild for projection, xr: {:?} cam: {:?}",
				self.projection,
				projection
			);
			return true;
		}
		let cam_viewport = camera.viewport.clone().unwrap();
		if !self.viewport.is_equal(&cam_viewport) {
			log!(
				"rebuild for projection, xr: {:?} cam: {:?}",
				self.viewport,
				&cam_viewport
			);
			return true;
		}
		false
	}
}
