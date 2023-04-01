use bevy::{
	ecs::{
		system::{EntityCommands},
	},
	prelude::*,
	render::camera::Viewport,
};
use web_sys::*;


#[derive(Component)]
pub struct LeftHand;


#[derive(Component)]
pub struct RightHand;

#[derive(Component)]
pub struct NoneHand;

pub fn insert_input_handedness(
	entity: &mut EntityCommands,
	handedness: XrHandedness,
) {
	match handedness {
		XrHandedness::Left => entity.insert(LeftHand),
		XrHandedness::Right => entity.insert(RightHand),
		_ => entity.insert(NoneHand),
	};
}
pub fn insert_view_handedness(
	entity: &mut EntityCommands,
	viewport: Viewport,
	num_views: usize,
) {
	if num_views == 0 {
		entity.insert(NoneHand)
	} else if viewport.physical_position.x == 0 {
		entity.insert(LeftHand)
	} else {
		entity.insert(RightHand)
	};
}
