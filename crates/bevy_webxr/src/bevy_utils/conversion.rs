use bevy::{prelude::*, render::extract_component::ExtractComponent};
use web_sys::*;




pub fn dom_point_to_vec3(dom_point: &DomPointReadOnly) -> Vec3 {
	Vec3 {
		x: dom_point.x() as f32,
		y: dom_point.y() as f32,
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
pub fn dom_point_to_quat_inverted(dom_point: &DomPointReadOnly) -> Quat {
	Quat {
		x: dom_point.x() as f32,
		y: dom_point.y() as f32,
		z: -dom_point.z() as f32,
		w: -dom_point.w() as f32,
	}
}
