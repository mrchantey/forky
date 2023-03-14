use std::{collections::hash_map::DefaultHasher, hash::Hasher};

use crate::*;
use bevy::{prelude::*, render::camera::Viewport};
use web_sys::*;

#[derive(Component)]
pub struct BevyXrView {
	pub hash: u64,
	pub pose: bevy_utils::Pose,
	pub viewport: Viewport,
	pub projection: bevy_utils::RawProjection,
	// pub projection: Vec<f32>,
	// pub
}


impl BevyXrView {
	pub fn new(view: &XrView, gl_layer: &XrWebGlLayer) -> Self {
		let pose: bevy_utils::Pose = view.transform().into();
		let viewport = gl_layer.get_viewport(view).unwrap();
		let viewport = bevy_utils::view_viewport(&viewport);
		let projection = view.projection_matrix();
		let projection =
			bevy_utils::RawProjection::from_vec_inverted(&projection);
		// let mut projection = bevy_utils::projection_from_vec(&projection);
		//fov seems to always be negative, but no issue
		// projection.fov *= -1.;
		// projection.near = 0.01;
		// projection.far = 1000.0;
		//otherwise bevy inverts the ar? ie 2 becomes 0.5
		// projection.aspect_ratio = 1. / projection.aspect_ratio;

		Self {
			hash: Self::get_hash(view, gl_layer),
			pose,
			viewport,
			projection,
		}
	}

	pub fn get_hash(view: &XrView, gl_layer: &XrWebGlLayer) -> u64 {
		let mut hasher = DefaultHasher::new();
		let projection = view.projection_matrix();
		hash_vec_f32(&projection, &mut hasher);
		let viewport = gl_layer.get_viewport(view).unwrap();
		hash_viewport(&viewport, &mut hasher);
		hasher.finish()
	}

	pub fn needs_rebuild(
		&self,
		// camera: &Camera,
		// projection: &PerspectiveProjection,
	) -> bool {
		//TODO use hash not camera
		// if !self.projection.is_equal(projection) {
		// 	log!(
		// 		"rebuild for projection, current: {:?} next: {:?}",
		// 		projection,
		// 		self.projection,
		// 	);
		// 	return true;
		// }
		// let cam_viewport = camera.viewport.clone().unwrap();
		// if !self.viewport.is_equal(&cam_viewport) {
		// 	log!(
		// 		"rebuild for viewport, current: {:?} next: {:?}",
		// 		&cam_viewport,
		// 		self.viewport,
		// 	);
		// 	return true;
		// }
		false
	}
}
