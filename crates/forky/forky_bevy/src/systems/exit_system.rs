use bevy::app::AppExit;
use bevy::core::FrameCount;
use bevy::prelude::*;


pub fn exit_system(mut exit: EventWriter<AppExit>) { exit.send(AppExit); }
pub fn exit_in_frames(
	count: u32,
) -> impl Fn(Res<FrameCount>, EventWriter<AppExit>) {
	move |frames, mut exit| {
		if frames.0 >= count - 1 {
			exit.send(AppExit);
		}
	}
}


pub fn close_on_esc(
	mut commands: Commands,
	focused_windows: Query<(Entity, &Window)>,
	input: Res<ButtonInput<KeyCode>>,
) {
	for (window, focus) in focused_windows.iter() {
		if !focus.focused {
			continue;
		}

		if input.just_pressed(KeyCode::Escape) {
			commands.entity(window).despawn();
		}
	}
}
