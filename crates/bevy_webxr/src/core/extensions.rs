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
