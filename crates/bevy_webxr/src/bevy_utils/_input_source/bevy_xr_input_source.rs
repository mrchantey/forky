use crate::*;
use bevy::prelude::*;
use js_sys::JsString;
use std::{
	collections::{hash_map::DefaultHasher, HashMap},
	hash::{Hash, Hasher},
};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::*;

#[derive(Resource, Deref, DerefMut)]
pub struct BevyInputSourceLookup(pub HashMap<u64, BevyXrInputSource>);

#[derive(Component, Deref, DerefMut)]
pub struct InputSourceHash(pub u64);

#[derive(Component)]
pub struct BevyXrInputSource {
	pub hash: u64,
	pub grip_pose: bevy_utils::Pose,
}

impl BevyXrInputSource {
	pub fn new(
		input_source: XrInputSource,
		frame: &XrFrame,
		reference_space: &XrReferenceSpace,
	) -> Self {
		let grip_pose = frame
			.get_pose(&input_source.grip_space().unwrap(), &reference_space)
			.unwrap();

		Self {
			hash: Self::get_hash(&input_source),
			grip_pose: grip_pose.into(),
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
}
/*
handedness: right
profile: oculus-touch
profile: generic-trigger-squeeze-thumbstick
*/
