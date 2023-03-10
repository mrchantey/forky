use crate::*;
use bevy::prelude::*;
use extend::ext;
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

#[ext(name = OptI32X)]
pub impl App {
	fn __(&mut self) -> &mut Self { self }
	fn run_webxr(mut self) { bevy_utils::run_bevy_webxr(self); }
}
