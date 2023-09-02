use bevy::prelude::*;
use extend::ext;
use std::time::Duration;
use bevy::utils::Instant;

#[ext(name=AppExtSweet)]
pub impl App {
	fn run_once(&mut self)->&mut Self{
		self.finish();
		self.update();
		self
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
		time.update_with_instant(
			last_update + duration,
		);
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