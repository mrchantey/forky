use web_sys::HtmlElement;

use crate::Matcher;

pub fn expect_body() -> Matcher<HtmlElement> {
	let document = web_sys::window().unwrap().document().unwrap();
	let body = document.body().unwrap();
	Matcher::new(body)
}
