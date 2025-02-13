use crate::prelude::*;
use bevy::prelude::*;
use extend::ext;
use forky_core::utils::RcCell;

#[ext]
pub impl App {
	fn __(&mut self) -> &mut Self { self }
	fn with_app_res(self) -> RcCell<Self> { AppRes::init(self) }

	#[cfg(target_arch = "wasm32")]
	#[must_use]
	fn run_on_animation_frame(mut self) -> forky_web::AnimationFrame {
		forky_web::AnimationFrame::new(move || {
			self.update();
		})
	}
}


#[cfg(test)]
mod test {
	use crate::prelude::*;
	use sweet::prelude::*;
	#[test]
	pub fn works() {
			let app = AppRes::new();
			let app = app.borrow_mut();
			expect(app.world().contains_non_send::<AppRes>()).to_be_true();
	}	
}