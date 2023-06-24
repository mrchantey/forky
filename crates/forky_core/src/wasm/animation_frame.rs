use crate::*;
use wasm_bindgen::prelude::*;
use web_sys::window;

pub struct AnimationFrame(pub RcCell<i32>);


impl AnimationFrame {
	pub fn new<F>(on_frame: F) -> Self
	where
		F: Fn() + 'static,
	{
		let f = rccell(None);
		let handle = rccell(0);
		let handle2 = handle.clone();
		let f2 = f.clone();
		*f2.borrow_mut() = Some(Closure::new(move || {
			on_frame();
			*handle2.borrow_mut() =
				request_animation_frame(f.borrow().as_ref().unwrap());
		}));
		*handle.borrow_mut() =
			request_animation_frame(f2.borrow().as_ref().unwrap());
		Self(handle)
	}

	pub fn cancel(&self) {
		window()
			.unwrap()
			.cancel_animation_frame(*self.0.borrow())
			.expect("failed to request animation frame");
	}
}

impl Drop for AnimationFrame {
	fn drop(&mut self) { self.cancel(); }
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) -> i32 {
	window()
		.unwrap()
		.request_animation_frame(f.as_ref().unchecked_ref())
		.expect("failed to request animation frame")
}
