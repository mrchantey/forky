use bevy::prelude::*;
use bevy_webxr::*;
use wasm_bindgen::prelude::*;

fn main() {
	set_panic_hook();
	let mut app = App::new();
	app
		// .add_plugins(DefaultPlugins.build().disable::<WinitPlugin>())
		.add_plugins(demo::DemoScenePlugin)
		.run();
	// .update();

	let update = Closure::<dyn FnMut()>::new(move || {
		// log!("update!");
		app.update();
	});

	web_sys::window()
		.unwrap()
		.set_interval_with_callback_and_timeout_and_arguments_0(
			update.as_ref().unchecked_ref(),
			1,
		)
		.unwrap();
	update.forget(); //terrible, memory leak
}
