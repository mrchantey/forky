use bevy::app::AppExit;
use bevy::core::FrameCount;
use bevy::prelude::*;

pub fn exit_system(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
}
pub fn exit_in_frames(count: u32) -> impl Fn(Res<FrameCount>, EventWriter<AppExit>) {
    move |frames, mut exit| {
        if frames.0 >= count - 1 {
            exit.send(AppExit::Success);
        }
    }
}

pub fn close_on_esc(mut exit: EventWriter<AppExit>, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
