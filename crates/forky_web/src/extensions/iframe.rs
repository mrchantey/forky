use crate::HtmlEventListener;
use extend::ext;
use wasm_bindgen::JsCast;
use web_sys::HtmlIFrameElement;

#[ext]
pub impl HtmlIFrameElement {
	fn x_reload(&self) {
		self.content_window().unwrap().location().reload().unwrap();
	}
	async fn x_reload_async(&self) {
		self.x_reload();
		let this = self.clone().unchecked_into();
		HtmlEventListener::wait_with_target("load", this).await;
	}
	// let window = self.content_window().unwrap();
	// HtmlEventListener::wait_with_target_and_while_listening(
	// 	"load",
	// 	this,
	// 	move || {
	// 		let location = window.location();
	// 		location.reload().unwrap();
	// 	},
	// )
	// .await;

	async fn x_wait_for_load(&self) {
		let this = self.clone().unchecked_into();
		HtmlEventListener::wait_with_target("load", this).await;
	}
}
