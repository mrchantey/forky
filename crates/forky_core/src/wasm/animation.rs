use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::window;

pub fn on_animation_frame<F>(on_frame: F)
where
	F: Fn() + 'static,
	// F: FnMut() + 'static,
{
	let f = Rc::new(RefCell::new(None));
	let g = f.clone();
	*g.borrow_mut() = Some(Closure::new(move || {
		on_frame();
		request_animation_frame(f.borrow().as_ref().unwrap());
	}));
	request_animation_frame(g.borrow().as_ref().unwrap());
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
	window()
		.unwrap()
		.request_animation_frame(f.as_ref().unchecked_ref())
		.expect("failed to request animation frame");
}
