use bevy::prelude::*;
use bevy_webxr::*;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

fn main() {}

#[wasm_bindgen]
pub struct Runner {
	app: Arc<Mutex<App>>,
}

#[wasm_bindgen]
impl Runner {
	#[wasm_bindgen(constructor)]
	pub fn new() -> Runner {
		set_panic_hook();
		let mut app = App::new();
		app
			// .add_plugins(DefaultPlugins.build().disable::<WinitPlugin>())
			.add_plugin(demo::DemoScenePlugin);
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
			)
			.unwrap();
		update.forget(); //terrible, memory leak

		Runner {
			app: Arc::clone(&app_arc),
		}
	}
	pub fn set_speed(&mut self, speed: f32) {
		log!("setting speed!");
		self.app
			.lock()
			.unwrap()
			.insert_resource(demo::Speed::new(speed));
	}
}
