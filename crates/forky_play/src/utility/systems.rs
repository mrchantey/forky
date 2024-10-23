use bevy::app::AppExit;
use bevy::prelude::*;
// use enigo::*;


pub fn create_exit_after_system(
	secs: f32,
) -> impl Fn(EventWriter<AppExit>, Res<Time>) {
	move |mut exit, time| {
		if time.elapsed_secs() > secs {
			exit.send(AppExit::Success);
		}
	}
}

pub fn surrender_focus() {
	// let mut enigo = Enigo::new();
	// enigo.key_down(Key::Alt);
	// enigo.key_click(Key::Escape);
	// enigo.key_up(Key::Alt);
}
