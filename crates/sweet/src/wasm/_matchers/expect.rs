use web_sys::*;

use crate::Matcher;


pub fn expect_body() -> Matcher<Element> {
	let document = web_sys::window().unwrap().document().unwrap();
	let body = document.body().unwrap();
	Matcher::new(body)
}
