// #![cfg(web_sys_unstable_apis)]

use crate::{core::*};
use bevy::{prelude::*};
use std::{
	sync::{Arc, Mutex},
};
use web_sys::*;


#[derive(Resource)]
pub struct XrFrameRes {}

impl XrFrameRes {
	pub fn new(frame: XrFrame) -> XrFrameRes { 
		// let viewer = frame.get_viewer_pose(XrReferenceSpace::)	
		let session = frame.session();
		let _reference_space = session.request_reference_space(XrReferenceSpaceType::Local);
		XrFrameRes {}
	}
}

// pub fn run_bevy_webxr(app: App) {}
pub fn run_bevy_webxr(app: App) {
	set_panic_hook();
	let app = Arc::new(Mutex::new(app));
	let app = Arc::clone(&app);
	init_and_run_xr(move |_time: f64, frame: XrFrame| {
		let mut app = app.lock().unwrap();
		app.insert_resource(XrFrameRes::new(frame));
		app.update();
	});
}
