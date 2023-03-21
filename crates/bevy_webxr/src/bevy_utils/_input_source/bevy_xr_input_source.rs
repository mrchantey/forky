use crate::*;
use bevy::prelude::*;
use js_sys::JsString;
use std::{
	collections::{hash_map::DefaultHasher, HashMap},
	hash::{Hash, Hasher},
};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::*;

#[derive(Component)]
pub struct BevyXrInputSource {
	pub hash: u64,
}

impl BevyXrInputSource {
	pub fn new(input_source: &XrInputSource) -> Self {
		Self {
			hash: Self::get_hash(&input_source),
		}
	}

	pub fn get_hash(input_source: &XrInputSource) -> u64 {
		let mut hasher = DefaultHasher::new();
		hash_js_value(&input_source.handedness(), &mut hasher);
		for profile in input_source.profiles().iter() {
			hash_js_value(&profile, &mut hasher);
		}
		hasher.finish()
	}

	pub fn grip_pose(
		source: &XrInputSource,
		frame: &XrFrame,
		reference_space: &XrReferenceSpace,
	) -> bevy_utils::Pose {
		frame
			.get_pose(&source.grip_space().unwrap(), &reference_space)
			.unwrap()
			.into()
	}
}
/*
handedness: right
profile: oculus-touch
profile: generic-trigger-squeeze-thumbstick
*/
