use crate::HtmlEventListener;
use extend::ext;
use web_sys::HtmlIFrameElement;

#[ext]
pub impl HtmlIFrameElement {
	fn x_reload(&self) {
		self.content_window().unwrap().location().reload().unwrap();
	}
	// async fn x_reload_async(&self) {
	// 	let window = self.content_window().unwrap();
	// 	let location = window.location();
	// 	location.reload().unwrap();
	// 	HtmlEventListener::wait_with_window("load", window).await;
	// 	// HtmlEventListener::wait_with_window("click", window).await;
	// }
	async fn x_reload_async(&self) {
		let window = self.content_window().unwrap();
		HtmlEventListener::wait_with_window_and_while_listening(
			"load",
			window.clone(),
			move || {
				let location = window.location();
				location.reload().unwrap();
			},
		)
		.await;
		// HtmlEventListener::wait_with_window("click", window).await;
	}
}
