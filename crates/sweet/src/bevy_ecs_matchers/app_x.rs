use bevy_app::prelude::*;
use bevy_time::prelude::*;
use extend::ext;
use std::time::Duration;
use std::time::Instant;


#[ext(name=EcsAppExtSweet)]
/// Ease-of-use extensions for `bevy::App`
pub impl App {
	fn insert_test_timer(&mut self) -> &mut Self {
		let mut time = Time::default();
		let now = Instant::now();
		time.update_with_instant(now);
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
