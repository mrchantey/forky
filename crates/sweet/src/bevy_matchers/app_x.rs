use bevy::app::AppExit;
use bevy::core::FrameCount;
// use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy::winit::WinitSettings;
use extend::ext;

fn exit_system(mut exit: EventWriter<AppExit>) { exit.send(AppExit); }
fn exit_in_frames(
	count: u32,
) -> impl Fn(Res<FrameCount>, EventWriter<AppExit>) {
	move |frames, mut exit| {
		if frames.0 >= count - 1 {
			exit.send(AppExit);
		}
	}
}

#[ext(name=AppExtSweet)]
#[doc(cfg(feature = "bevy"))]
/// Ease-of-use extensions for `bevy::App`
pub impl App {
	fn run_once(&mut self) {
		self.insert_resource(WinitSettings {
			return_from_run: true,
			..default()
		})
		// dont run last, avoid race with update_frame_count
		.add_systems(First, exit_system)
		.run();
		// self.finish();
		// self.update();
	}
	fn run_for_frames(&mut self, count: u32) {
		self.insert_resource(WinitSettings {
			return_from_run: true,
			..default()
		})
		.add_systems(Last, exit_in_frames(count))
		.run();
	}
}
