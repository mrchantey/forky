use forky_web::prelude::*;
use web_sys::Event;


fn main() {
	let _listener = HtmlEventListener::new("click", |_e: Event| {
		web_sys::console::log_1(&"clicked..".into());
	});
}
