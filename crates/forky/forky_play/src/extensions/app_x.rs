use super::*;
use crate::utility;
use bevy::prelude::*;
use extend::ext;
use forky_core::RcCell;

#[ext]
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

	fn with_app_res(self) -> RcCell<Self> { AppRes::init(self) }

	#[cfg(target_arch = "wasm32")]
	fn run_on_animation_frame(mut self) -> forky_web::AnimationFrame {
		forky_web::AnimationFrame::new(move || {
			self.update();
		})
	}
}
