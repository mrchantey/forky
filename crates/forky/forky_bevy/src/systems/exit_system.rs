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
