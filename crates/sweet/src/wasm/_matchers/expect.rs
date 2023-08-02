use crate::Matcher;
use forky_web::*;
use web_sys::*;

pub fn expect_body() -> Matcher<HtmlElement> {
	Matcher::new(Document::x_body())
}
