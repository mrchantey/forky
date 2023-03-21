use std::{collections::hash_map::DefaultHasher, hash::Hasher};

use crate::*;
use bevy::{prelude::*, render::camera::Viewport, utils::HashMap};
use derive_deref::{Deref, DerefMut};
use web_sys::*;

#[derive(Component, Debug, Clone)]
pub struct BevyXrView {
	pub hash: u64,
	pub viewport: Viewport,
	pub projection: bevy_utils::RawProjection,
}

impl BevyXrView {
	pub fn new(view: &XrView, gl_layer: &XrWebGlLayer) -> Self {
		let viewport = gl_layer.get_viewport(view).unwrap();
		let viewport = bevy_utils::view_viewport(&viewport);
		let projection = view.projection_matrix();
		let projection =
			bevy_utils::RawProjection::from_vec_inverted(&projection);

		Self {
			hash: Self::get_hash(view, gl_layer),
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
}
