use bevy::{app::AppExit, prelude::*, window::WindowCommand};
use enigo::*;
use std::{thread, time};

// basic_plugin_::
use super::KEY_COMMAND;
type Sys = fn(EventWriter<AppExit>);

pub fn create_exit_after_system(secs: u64) -> impl Fn(EventWriter<AppExit>, Res<Time>) {
	move |mut exit, time| {
		if time.seconds_since_startup() > secs as f64 {
			exit.send(AppExit);
		}
		// thread::sleep(time::Duration::from_secs(secs));
	}
}

pub fn exit_system(mut exit: EventWriter<AppExit>) {
	thread::sleep(time::Duration::from_secs(3));

	exit.send(AppExit);
}

pub fn surrender_focus() {
	let mut enigo = Enigo::new();
	enigo.key_down(Key::Alt);
	enigo.key_click(Key::Escape);
	enigo.key_up(Key::Alt);
}
