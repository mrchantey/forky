use crate::*;
use bevy::{prelude::*, render::camera::Viewport};
use extend::ext;
use wasm_bindgen::JsValue;
use web_sys::*;

pub trait GetFramebuffer {
	fn framebuffer_unwrapped(&self) -> WebGlFramebuffer;
}

impl GetFramebuffer for XrWebGlLayer {
	fn framebuffer_unwrapped(&self) -> WebGlFramebuffer {
		js_sys::Reflect::get(&self, &"framebuffer".into())
			.unwrap()
			.into()
	}
}

// impl GetPosition for XrRigidTransform {
// 	fn framebuffer_unwrapped(&self) -> DomPointReadOnly {
// 		js_sys::Reflect::get(&self, &"framebuffer".into())
// 			.unwrap()
// 			.into()
// 	}
// }

#[ext(name = AppX)]
pub impl App {
	/// Chaining helper, returns self
	fn __(&mut self) -> &mut Self { self }
	/// Custom runner that updates on the required XR requestAnimationFrame
	fn run_webxr(mut self) { bevy_utils::run_bevy_webxr(self); }
}
#[ext(name = WorldX)]
pub impl World {
	/// Chaining helper, returns self
	fn __(&mut self) -> &mut Self { self }
}
#[ext(name = Mat4X)]
pub impl Mat4 {
	fn is_equal(&self, arr: &Vec<f32>, eps: f32) -> bool {
		mat4_equal(self, arr, eps)
	}
}

#[ext(name = Vec4X)]
pub impl Vec4 {
	fn is_equal(&self, arr: &Vec<f32>, eps: f32) -> bool {
		vec4_equal(self, arr, eps)
	}
}

#[rustfmt::skip]
#[ext(name = PerspectiveProjectionX)]
pub impl PerspectiveProjection {
	fn is_equal(&self, other: &PerspectiveProjection) -> bool {
		f32_equal(self.fov, other.fov, EPSILON_POJECTION)
			&& f32_equal(self.aspect_ratio,other.aspect_ratio,EPSILON_POJECTION) 
			&& f32_equal(self.near, other.near, EPSILON_POJECTION)
			&& f32_equal(self.far, other.far, EPSILON_POJECTION)
	}
}
#[ext(name = ViewportX)]
pub impl Viewport {
	fn is_equal(&self, other: &Viewport) -> bool {
		self.physical_position == other.physical_position
			&& self.physical_size == other.physical_size
			&& f32_equal(self.depth.start, other.depth.start, EPSILON_POJECTION)
			&& f32_equal(self.depth.end, other.depth.end, EPSILON_POJECTION)
	}
}
#[ext(name = ArrayX)]
pub impl js_sys::Array {
	fn to_vec_typed<T>(&self) -> Vec<T>
	where
		T: From<JsValue>,
	{
		self.iter().map(|x| x.into()).collect::<Vec<T>>()
	}
}

