use std::sync::{Arc, Mutex};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::DeviceMotionEvent;


#[derive(Default)]
pub struct DeviceAccelerometer {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

impl DeviceAccelerometer {
	pub fn new() -> Arc<Mutex<DeviceAccelerometer>> {
		let acc = Arc::new(Mutex::new(DeviceAccelerometer::default()));

		let acc2 = acc.clone();
		let closure = Closure::wrap(Box::new(move |event: DeviceMotionEvent| {
			let acc_event = event.acceleration().unwrap();
			let mut acc = acc2.lock().unwrap();
			acc.x = acc_event.x().unwrap_or(0.);
			acc.y = acc_event.y().unwrap_or(0.);
			acc.z = acc_event.z().unwrap_or(0.);
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
