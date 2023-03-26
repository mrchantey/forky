use bevy::{app::AppExit, prelude::*};
// use enigo::*;
use std::{thread, time};


pub fn spawn_camera_at_position(position: Vec3) -> impl FnMut(Commands) {
	move |mut commands| {
		commands.spawn(Camera3dBundle {
			transform: Transform::from_translation(position)
				.looking_at(Vec3::ZERO, Vec3::Y),
			..default()
		});
	}
}


pub fn create_exit_after_system(
	secs: f64,
) -> impl Fn(EventWriter<AppExit>, Res<Time>) {
	move |mut exit, time| {
		if time.elapsed_seconds_f64() > secs {
			exit.send(AppExit);
		}
	}
}

pub fn exit_system(mut exit: EventWriter<AppExit>) {
	thread::sleep(time::Duration::from_secs(3));

	exit.send(AppExit);
}

pub fn surrender_focus() {
	// let mut enigo = Enigo::new();
	// enigo.key_down(Key::Alt);
	// enigo.key_click(Key::Escape);
	// enigo.key_up(Key::Alt);
}
