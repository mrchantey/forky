use bevy::prelude::*;
use extend::ext;
use forky_core::*;
#[derive(Deref, DerefMut)]
pub struct AppRes(pub RcCell<App>);

impl AppRes {
	pub fn new() -> RcCell<App> { Self::init(App::new()) }


	pub fn build(func: impl FnOnce(&mut App)) -> RcCell<App> {
		let mut app = App::new();
		func(&mut app);
		Self::init(app)
	}

	pub fn init(app: App) -> RcCell<App> {
		let app = rccell(app);
		let app2 = app.clone();
		app.borrow_mut().insert_non_send_resource(AppRes(app2));
		app
	}
}

#[ext]
pub impl RcCell<App> {
	#[cfg(target_arch = "wasm32")]
	fn run_on_animation_frame(self) -> forky_web::AnimationFrame {
		forky_web::AnimationFrame::new(move || {
			self.borrow_mut().update();
		})
	}

	#[cfg(target_arch = "wasm32")]
	fn run_forever(self) -> impl std::future::Future<Output = ()> {
		async {
			let _frame = self.run_on_animation_frame();
			forky_web::loop_forever().await;
		}
	}

	#[cfg(target_arch = "wasm32")]
	fn run_while_mounted(self) {
		leptos::spawn_local(async move {
			let mounted = forky_web::is_mounted();
			let _frame = forky_web::AnimationFrame::new(move || {
				if mounted() {
					self.borrow_mut().update();
				}
			});
			forky_web::loop_while_mounted().await;
		})
	}
}
