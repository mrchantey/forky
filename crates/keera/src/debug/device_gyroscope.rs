use std::sync::{Arc, Mutex};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::DeviceMotionEvent;


#[derive(Default)]
pub struct DeviceGyroscope {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl DeviceGyroscope {
	pub fn new() -> Arc<Mutex<DeviceGyroscope>> {
		let acc = Arc::new(Mutex::new(DeviceGyroscope::default()));

		let acc2 = acc.clone();
		let closure = Closure::wrap(Box::new(move |event: DeviceMotionEvent| {
			let rot_event = event.rotation_rate().unwrap();
			let mut acc = acc2.lock().unwrap();
			acc.x = rot_event.alpha().unwrap_or(0.);
			acc.y = rot_event.beta().unwrap_or(0.);
			acc.z = rot_event.gamma().unwrap_or(0.);
		}) as Box<dyn FnMut(_)>);

		web_sys::window()
			.unwrap()
			.add_event_listener_with_callback(
				"devicemotion",
				closure.as_ref().unchecked_ref(),
			)
			.unwrap();
		closure.forget();
		acc
	}
}
