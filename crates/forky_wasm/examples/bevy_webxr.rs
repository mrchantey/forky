use forky_wasm::*;
use bevy::prelude::*;
use forky_play::app::SimplePlugin;

fn main()
{
	let mut app = App::new();
	app.add_plugins(DefaultPlugins)
// .add_plugins(DefaultPlugins.build().disable::<WinitPlugin>())
	.add_plugin(SimplePlugin);
	core::run_bevy_webxr(app);
}