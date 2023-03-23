use crate::utility;
use bevy::prelude::*;
use extend::ext;
use std::time::{Duration, Instant};

#[ext(name = AppX)]
pub impl App {
	fn __(&mut self) -> &mut Self { self }
	fn forky_surrender_focus(&mut self) -> &mut Self {
		self.add_startup_system(utility::surrender_focus);
		self
	}
	fn forky_exit_after(&mut self, secs: f64) -> &mut Self {
		self.add_system(utility::create_exit_after_system(secs));
		self
	}
	fn insert_test_timer(&mut self) -> &mut Self {
		let mut time = Time::default();
		let start_instant = Instant::now();
		time.update_with_instant(start_instant);
		self.insert_resource(time);
		self
	}
	fn update_with_tick(&mut self, secs: f32) -> &mut Self {
		let mut time = self.world.resource_mut::<Time>();
		let last_update = time.last_update().unwrap();
		time.update_with_instant(
			last_update + Duration::from_millis((secs * 1000.) as u64),
		);
		self.update();
		self
	}
}
