use super::*;
use crate::*;
use bevy::prelude::*;
use forky_core::math::*;
use forky_core::*;

fn _vertical(cell_width: f32, wall_width: f32) -> Vec<Transform> {
	vec![Transform::from_scale_xyz(wall_width, 1., cell_width)]
}
fn _half_vertical(cell_width: f32, wall_width: f32) -> Vec<Transform> {
	let h_width = wall_width / 2.;
	let q_width = wall_width / 4.;
	let height = cell_width * 0.5 + h_width;
	let pos = cell_width * 0.25 + q_width;

	vec![Transform::from_xyz(pos, 0., 0.).with_scale_xyz(height, 1., wall_width)]
}
fn _tee(cell_width: f32, wall_width: f32) -> Vec<Transform> {
	let h_width = wall_width / 2.;
	let q_width = wall_width / 4.;
	let short_height = cell_width * 0.5 - h_width;
	let short_pos = cell_width * 0.25 + q_width;

	vec![
		Transform::from_scale_xyz(wall_width,1.,cell_width),
		Transform::from_xyz(short_pos, 0., 0.).with_scale_xyz(
			short_height,
			1.,
			wall_width,
		),
	]
}
fn _cross(cell_width: f32, wall_width: f32) -> Vec<Transform> {
	let h_width = wall_width / 2.;
	let q_width = wall_width / 4.;
	let short_height = cell_width * 0.5 - h_width;
	let short_pos = cell_width * 0.25 + q_width;

	vec![
		Transform::from_scale_xyz(wall_width,1.,cell_width),
		Transform::from_xyz(short_pos, 0., 0.).with_scale_xyz(
			short_height,
			1.,
			wall_width,
		),
		Transform::from_xyz(-short_pos, 0., 0.).with_scale_xyz(
			short_height,
			1.,
			wall_width,
		),
	]
}

fn _corner(cell_width: f32, wall_width: f32) -> Vec<Transform> {
	let h_width = wall_width / 2.;
	let q_width = wall_width / 4.;

	let long_height = cell_width * 0.5 + h_width;
	let long_pos = cell_width * 0.25 - q_width;

	let short_height = cell_width * 0.5 - h_width;
	let short_pos = cell_width * 0.25 + q_width;
	vec![
		//long
		Transform::from_xyz(0., 0., long_pos).with_scale(Vec3::new(
			wall_width,
			1.,
			long_height,
		)),
		//short
		Transform::from_xyz(short_pos, 0., 0.)
			.with_scale(Vec3::new(wall_width, 1., short_height))
			.with_rotation_y(QUARTER_TAU),
	]
}


pub fn horizontal(
	cell_width: f32,
	wall_width: f32,
) -> (Transform, Vec<Transform>) {
	(
		Transform::from_rotation_y(QUARTER_TAU),
		_vertical(cell_width, wall_width),
	)
}
pub fn vertical(
	cell_width: f32,
	wall_width: f32,
) -> (Transform, Vec<Transform>) {
	(Transform::default(), _vertical(cell_width, wall_width))
}

pub fn none(cell_width: f32, wall_width: f32) -> (Transform, Vec<Transform>) {
	(Transform::default(), Vec::default())
}
pub fn cross(cell_width: f32, wall_width: f32) -> (Transform, Vec<Transform>) {
	(Transform::default(), _cross(cell_width, wall_width))
}

pub fn top_left(
	cell_width: f32,
	wall_width: f32,
) -> (Transform, Vec<Transform>) {
	(
		Transform::from_rotation_y(0.),
		_corner(cell_width, wall_width),
	)
}
pub fn top_right(
	cell_width: f32,
	wall_width: f32,
) -> (Transform, Vec<Transform>) {
	(
		Transform::from_rotation_y(THREE_QUARTER_TAU),
		_corner(cell_width, wall_width),
	)
}
pub fn bottom_left(
	cell_width: f32,
	wall_width: f32,
) -> (Transform, Vec<Transform>) {
	(
		Transform::from_rotation_y(QUARTER_TAU),
		_corner(cell_width, wall_width),
	)
}
pub fn bottom_right(
	cell_width: f32,
	wall_width: f32,
) -> (Transform, Vec<Transform>) {
	(
		Transform::from_rotation_y(HALF_TAU),
		_corner(cell_width, wall_width),
	)
}

pub fn left_tee(
	cell_width: f32,
	wall_width: f32,
) -> (Transform, Vec<Transform>) {
	(Transform::from_rotation_y(0.), _tee(cell_width, wall_width))
}
pub fn top_tee(
	cell_width: f32,
	wall_width: f32,
) -> (Transform, Vec<Transform>) {
	(
		Transform::from_rotation_y(THREE_QUARTER_TAU),
		_tee(cell_width, wall_width),
	)
}
pub fn right_tee(
	cell_width: f32,
	wall_width: f32,
) -> (Transform, Vec<Transform>) {
	(
		Transform::from_rotation_y(HALF_TAU),
		_tee(cell_width, wall_width),
	)
}
pub fn bottom_tee(
	cell_width: f32,
	wall_width: f32,
) -> (Transform, Vec<Transform>) {
	(
		Transform::from_rotation_y(QUARTER_TAU),
		_tee(cell_width, wall_width),
	)
}

pub fn horizontal_left(
	cell_width: f32,
	wall_width: f32,
) -> (Transform, Vec<Transform>) {
	(
		Transform::from_rotation_y(HALF_TAU),
		_half_vertical(cell_width, wall_width),
	)
}
pub fn vertical_top(
	cell_width: f32,
	wall_width: f32,
) -> (Transform, Vec<Transform>) {
	(
		Transform::from_rotation_y(QUARTER_TAU),
		_half_vertical(cell_width, wall_width),
	)
}
pub fn horizontal_right(
	cell_width: f32,
	wall_width: f32,
) -> (Transform, Vec<Transform>) {
	(
		Transform::from_rotation_y(0.),
		_half_vertical(cell_width, wall_width),
	)
}
pub fn vertical_bottom(
	cell_width: f32,
	wall_width: f32,
) -> (Transform, Vec<Transform>) {
	(
		Transform::from_rotation_y(THREE_QUARTER_TAU),
		_half_vertical(cell_width, wall_width),
	)
}

pub fn from_u8(val: u8) -> fn(f32, f32) -> (Transform, Vec<Transform>) {
	match val {
		u8_shape::NONE => none,
		u8_shape::VERTICAL => vertical,
		u8_shape::HORIZONTAL => horizontal,
		u8_shape::CROSS => cross,

		u8_shape::TOP_LEFT => top_left,
		u8_shape::TOP_RIGHT => top_right,
		u8_shape::BOTTOM_RIGHT => bottom_right,
		u8_shape::BOTTOM_LEFT => bottom_left,

		u8_shape::TOP_TEE => top_tee,
		u8_shape::BOTTOM_TEE => bottom_tee,
		u8_shape::LEFT_TEE => left_tee,
		u8_shape::RIGHT_TEE => right_tee,

		u8_shape::HORIZONTAL_LEFT => horizontal_left,
		u8_shape::HORIZONTAL_RIGHT => horizontal_right,
		u8_shape::VERTICAL_TOP => vertical_top,
		u8_shape::VERTICAL_BOTTOM => vertical_bottom,

		u8_shape::DIAG_CROSS => none,
		u8_shape::DIAG_TL_BR => none,
		u8_shape::DIAG_TR_BL => none,
		_ => none,
	}
}
