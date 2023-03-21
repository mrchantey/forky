use std::f32::consts::PI;

use crate::*;
use bevy::{
	prelude::*,
	render::{camera::Viewport, extract_component::ExtractComponent},
};
use web_sys::*;


pub fn view_viewport(viewport: &XrViewport) -> Viewport {
	Viewport {
		physical_position: UVec2::new(viewport.x() as u32, viewport.y() as u32),
		physical_size: UVec2::new(
			viewport.width() as u32,
			viewport.height() as u32,
		),
		..default()
	}
}

//chatgpt
pub fn view_projection(view: &XrView) -> PerspectiveProjection {
	let proj_matrix = view.projection_matrix();
	projection_from_vec(&proj_matrix)
}

pub fn dom_point_to_vec3(dom_point: &DomPointReadOnly) -> Vec3 {
	Vec3 {
		x: dom_point.x() as f32,
		y: dom_point.y() as f32,
		z: dom_point.z() as f32,
	}
}
pub fn dom_point_to_vec3_invert_x(dom_point: &DomPointReadOnly) -> Vec3 {
	Vec3 {
		x: -dom_point.x() as f32,
		y: dom_point.y() as f32,
		z: dom_point.z() as f32,
	}
}
pub fn dom_point_to_vec3_invert_y(dom_point: &DomPointReadOnly) -> Vec3 {
	Vec3 {
		x: dom_point.x() as f32,
		y: -dom_point.y() as f32,
		z: dom_point.z() as f32,
	}
}
pub fn dom_point_to_vec3_invert_z(dom_point: &DomPointReadOnly) -> Vec3 {
	Vec3 {
		x: dom_point.x() as f32,
		y: dom_point.y() as f32,
		z: -dom_point.z() as f32,
	}
}
pub fn dom_point_to_vec4(dom_point: &DomPointReadOnly) -> Vec4 {
	Vec4 {
		x: dom_point.x() as f32,
		y: dom_point.y() as f32,
		z: dom_point.z() as f32,
		w: dom_point.w() as f32,
	}
}
pub fn dom_point_to_quat(dom_point: &DomPointReadOnly) -> Quat {
	Quat {
		x: dom_point.x() as f32,
		y: dom_point.y() as f32,
		z: dom_point.z() as f32,
		w: dom_point.w() as f32,
	}
}
pub fn dom_point_to_quat_invert_yaw_roll(dom_point: &DomPointReadOnly) -> Quat {
	let mut euler = Quat {
		x: dom_point.x() as f32,
		y: dom_point.y() as f32,
		z: dom_point.z() as f32,
		w: dom_point.w() as f32,
	}
	.to_euler(EulerRot::XYZ);

	euler.1 = -euler.1;
	euler.2 = -euler.2;
	Quat::from_euler(EulerRot::XYZ, euler.0, euler.1, euler.2)
	// let inverted_yaw_roll_quat = Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), std::f32::consts::PI);

	// inverted_yaw_roll_quat * original_quat
}
// pub fn dom_point_to_quat_invert_pitch(dom_point: &DomPointReadOnly) -> Quat {
// 	Quat {
// 		x: -dom_point.x() as f32,
// 		y: dom_point.y() as f32,
// 		z: dom_point.z() as f32,
// 		w: dom_point.w() as f32,
// 	}
// }
// pub fn dom_point_to_quat_invert_yaw(dom_point: &DomPointReadOnly) -> Quat {
// 	Quat {
// 		x: dom_point.x() as f32,
// 		y: -dom_point.y() as f32,
// 		z: dom_point.z() as f32,
// 		w: dom_point.w() as f32,
// 	}
// }
// pub fn dom_point_to_quat_invert_roll(dom_point: &DomPointReadOnly) -> Quat {
// 	Quat {
// 		x: dom_point.x() as f32,
// 		y: dom_point.y() as f32,
// 		z: -dom_point.z() as f32,
// 		w: dom_point.w() as f32,
// 	}
// }

pub fn projection_from_vec(proj_matrix: &Vec<f32>) -> PerspectiveProjection {
	//not sure why but it needed inverting, *-1.
	let fov = f32::abs(2. * f32::atan(1.0 / proj_matrix[5]) * 180.0 / PI);

	let aspect_ratio = proj_matrix[5] / proj_matrix[0];
	// zNear = -proj_matrix[11] / (proj_matrix[10] - 1);
	// zFar = proj_matrix[11] / (proj_matrix[10] + 1);
	PerspectiveProjection {
		fov,
		aspect_ratio,
		near: 0.01,
		far: 1000.0,
	}
}
// pub fn projection_from_vec(proj_matrix: &Vec<f32>) -> PerspectiveProjection {
// 	let n = proj_matrix[14] / (proj_matrix[10] - 1.0);
// 	let r = n * (proj_matrix[2] - 1.0) / proj_matrix[0];
// 	let l = -r;
// 	let t = n * (proj_matrix[6] - 1.0) / proj_matrix[5];
// 	let b = -t;

// 	let aspect_ratio = (r - l) / (t - b);
// 	let fov = 2.0 * (t / n).atan() * 180.0 / std::f32::consts::PI;

// 	PerspectiveProjection {
// 		fov,
// 		aspect_ratio,
// 		near: 0.01,
// 		far: 1000.0,
// 	}
// }

/*

pub fn projection_from_mat(mat: &Mat4) -> PerspectiveProjection {
	let proj_matrix = mat.to_cols_array();
	projection_from_arr(&proj_matrix)
}


chatgpt

proj: PerspectiveProjection { fov: -0.39482957, aspect_ratio: 1.0001, near: 0.01, far: 1000.0 } cam: PerspectiveProjection { fov: -0.0040003946, aspect_ratio: 1.0002, near: 0.01, far: 0.0 }

[
	1.4349158, 0.0, 0.0, 0.0,
	0.0, 1.0, 0.0, 0.0,
	0.0, 0.0, -1.0002, -1.0,
	0.0, 0.0, -0.20002, 0.0
]



//three.js\src\renderers\webxr\WebXRManager.js#436
pub fn projection_from_vec(proj_matrix: &Vec<f32>) -> PerspectiveProjection {
	let near = proj_matrix[14] / (proj_matrix[10] - 1.);
	let far = proj_matrix[14] / (proj_matrix[10] + 1.);
	let topFov = (proj_matrix[9] + 1.) / proj_matrix[5];
	let bottomFov = (proj_matrix[9] - 1.) / proj_matrix[5];

	let fov = (proj_matrix[8] - 1.) / proj_matrix[0];
	let left = near * fov;
	// let width = proj_matrix[0];
	// let height = proj_matrix[5];
	// let near = proj_matrix[14];
	// let far = proj_matrix[15];

	// let aspect_ratio = width / height;
	// let vertical_fov = 2.0 * ((near / proj_matrix[5]).atan());
	PerspectiveProjection {
		fov: vertical_fov,
		aspect_ratio,
		near,
		far,
	}
}

pub fn projection_from_vec_broken(
	proj_matrix: &Vec<f32>,
) -> PerspectiveProjection {
	let width = proj_matrix[0];
	let height = proj_matrix[5];
	let near = proj_matrix[14];
	let far = proj_matrix[15];

	let aspect_ratio = width / height;
	let vertical_fov = 2.0 * ((near / proj_matrix[5]).atan());
	PerspectiveProjection {
		fov: vertical_fov,
		aspect_ratio,
		near,
		far,
	}
}


*/
