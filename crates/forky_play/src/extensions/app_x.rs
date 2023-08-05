use super::*;
use crate::utility;
use bevy::prelude::*;
use bevy::utils::Instant;
use extend::ext;
use forky_core::RcCell;
use std::time::Duration;

#[ext(name = AppX)]
pub impl App {
	fn __(&mut self) -> &mut Self { self }
	fn forky_surrender_focus(&mut self) -> &mut Self {
		self.add_systems(Startup, utility::surrender_focus);
		self
	}
	fn forky_exit_after(&mut self, secs: f64) -> &mut Self {
		self.add_systems(Update, utility::create_exit_after_system(secs));
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

	fn with_app_res(self) -> RcCell<Self> { AppRes::init(self) }

	#[cfg(target_arch = "wasm32")]
	fn run_on_animation_frame(mut self) -> forky_web::AnimationFrame {
		forky_web::AnimationFrame::new(move || {
			self.update();
		})
	}
}
