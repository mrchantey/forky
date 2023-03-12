use crate::*;
use bevy::{
	prelude::*,
	render::{camera::Viewport, extract_component::ExtractComponent},
};
use web_sys::*;


#[rustfmt::skip]
pub fn view_pose(view: &XrView) -> (Vec3, Quat) {
	let translation = dom_point_to_vec3_invert_y(&view.transform().position());
	let rotation = dom_point_to_quat_invert_pitch(&view.transform().orientation());
	(translation, rotation)
}

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
pub fn dom_point_to_vec3_invert_y(dom_point: &DomPointReadOnly) -> Vec3 {
	Vec3 {
		x: dom_point.x() as f32,
		y: -dom_point.y() as f32,
		z: dom_point.z() as f32,
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
pub fn dom_point_to_quat_invert_pitch(dom_point: &DomPointReadOnly) -> Quat {
	Quat {
		x: -dom_point.x() as f32,
		y: dom_point.y() as f32,
		z: dom_point.z() as f32,
		w: dom_point.w() as f32,
	}
}
