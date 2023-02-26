// #![cfg(web_sys_unstable_apis)]
use bevy::{prelude::*, winit::*};
use forky_play::{app::SimplePlugin, app::Speed, *};
use forky_wasm::*;
use std::{
	rc::Rc,
	sync::{Arc, Mutex},
	thread, time,
};
use wasm_bindgen::prelude::*;
use web_sys::*;


fn main() {}

#[wasm_bindgen]
pub struct Runner {
	app: Arc<Mutex<App>>,
}

#[wasm_bindgen]
impl Runner {
	#[wasm_bindgen(constructor)]
	pub fn new() -> Runner {
		core::set_panic_hook();
		let mut app = App::new();
		app.add_plugins(DefaultPlugins)
			// .add_plugins(DefaultPlugins.build().disable::<WinitPlugin>())
			.add_plugin(SimplePlugin);
		// .run();
		// .update();

		let app_arc = Arc::new(Mutex::new(app));
		let app_update = Arc::clone(&app_arc);
		let update = Closure::<dyn FnMut()>::new(move || {
			// log!("update!");
			app_update.lock().unwrap().update();
		});

		web_sys::window()
			.unwrap()
			.set_interval_with_callback_and_timeout_and_arguments_0(
				update.as_ref().unchecked_ref(),
				16,
			);
		update.forget(); //terrible, memory leak

		Runner {
			app: Arc::clone(&app_arc),
		}
	}
	pub fn set_speed(&mut self, speed: f32) {
		log!("setting speed!");
		self.app.lock().unwrap().insert_resource(Speed::new(speed));
	}
}
