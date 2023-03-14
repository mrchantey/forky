use bevy::prelude::*;

pub const EPSILON_POJECTION: f32 = 0.001;

pub fn f32_equal(a: f32, b: f32, eps: f32) -> bool { (a - b).abs() < eps }

pub fn mat4_equal(mat: &Mat4, arr: &[f32], eps: f32) -> bool {
	if arr.len() != 16 {
		return false; // Input vector does not have 16 elements
	}
	let x_equal = vec4_equal(&mat.x_axis, &arr[0..4], eps);
	let y_equal = vec4_equal(&mat.y_axis, &arr[4..8], eps);
	let z_equal = vec4_equal(&mat.z_axis, &arr[8..12], eps);
	let w_equal = vec4_equal(&mat.w_axis, &arr[12..16], eps);
	x_equal && y_equal && z_equal && w_equal
}

pub fn vec4_equal(vec1: &Vec4, arr: &[f32], eps: f32) -> bool {
	if arr.len() != 4 {
		return false;
	}
	f32_equal(vec1.x, arr[0], eps)
		&& f32_equal(vec1.y, arr[1], eps)
		&& f32_equal(vec1.z, arr[2], eps)
		&& f32_equal(vec1.w, arr[3], eps)
}
