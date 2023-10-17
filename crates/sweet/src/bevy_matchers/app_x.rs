use bevy::app::AppExit;
use bevy::core::FrameCount;
// use bevy::app::ScheduleRunnerPlugin;
use bevy::prelude::*;
use bevy::utils::Instant;
use bevy::winit::WinitSettings;
use extend::ext;
use std::time::Duration;

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

#[ext(name=AppExtSweet)]
#[doc(cfg(feature = "bevy"))]
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

	fn insert_test_timer(&mut self) -> &mut Self {
		let mut time = Time::default();
		let start_instant = Instant::now();
		time.update_with_instant(start_instant);
		self.insert_resource(time);
		self
	}
	fn update_with_duration(&mut self, duration: Duration) -> &mut Self {
		let mut time = self.world.resource_mut::<Time>();
		let last_update = time.last_update().unwrap();
		time.update_with_instant(last_update + duration);
		self.update();
		self
	}
	fn update_with_secs(&mut self, secs: u64) -> &mut Self {
		self.update_with_duration(Duration::from_secs(secs))
	}
	fn update_with_millis(&mut self, millis: u64) -> &mut Self {
		self.update_with_duration(Duration::from_millis(millis))
	}
}
