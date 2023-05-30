use std::sync::{Arc, Mutex};
use forky_core::math_f64::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{DeviceOrientationEvent, EventTarget};

#[derive(Default)]
pub struct DeviceOrientation {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}


impl DeviceOrientation {
	pub fn new() -> Arc<Mutex<DeviceOrientation>> {
		let gyro = Arc::new(Mutex::new(DeviceOrientation::default()));

		let gyro2 = gyro.clone();
		let closure =
			Closure::wrap(Box::new(move |event: DeviceOrientationEvent| {
				let alpha = event.alpha().unwrap_or(0.0) * DEG2RAD;
				let beta = event.beta().unwrap_or(0.0) * DEG2RAD;
				let gamma = event.gamma().unwrap_or(0.0) * DEG2RAD;

				let mut device = gyro2.lock().unwrap();
				device.x = alpha;
				device.y = beta;
				device.z = gamma;
			}) as Box<dyn FnMut(_)>);

		let target: EventTarget = web_sys::window().unwrap().into();
		target
			.add_event_listener_with_callback(
				"deviceorientation",
				closure.as_ref().unchecked_ref(),
			)
			.unwrap();
		closure.forget();
		gyro
	}
}
