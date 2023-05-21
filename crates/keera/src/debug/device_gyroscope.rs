use std::sync::{Arc, Mutex};

use forky_core::math_f64::*;
use js_sys::Date;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{DeviceOrientationEvent, EventTarget};

#[derive(Default)]
pub struct DeviceGyroscope {
	pub x: f64,
	pub y: f64,
	pub z: f64,
	pub rot_x: f64,
	pub rot_y: f64,
	pub rot_z: f64,
}


impl DeviceGyroscope {
	pub fn new() -> Arc<Mutex<DeviceGyroscope>> {
		let gyro = Arc::new(Mutex::new(DeviceGyroscope::default()));

		let gyro2 = gyro.clone();
		let mut last = Date::now();
		let closure =
			Closure::wrap(Box::new(move |event: DeviceOrientationEvent| {
				let alpha = event.alpha().unwrap_or(0.0) * DEG2RAD;
				let beta = event.beta().unwrap_or(0.0) * DEG2RAD;
				let gamma = event.gamma().unwrap_or(0.0) * DEG2RAD;

				let mut device = gyro2.lock().unwrap();
				let now = Date::now();
				// let delta_secs = (now - last) * 0.001;
				let delta_secs = 1.; //just for now for easy viewing
				last = now;
				//TODO factor in delta time
				device.x = angle_between(device.rot_x, alpha) * delta_secs;
				device.y = angle_between(device.rot_y, beta) * delta_secs;
				device.z = angle_between(device.rot_z, gamma) * delta_secs;
				device.rot_x = alpha;
				device.rot_y = beta;
				device.rot_z = gamma;
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
