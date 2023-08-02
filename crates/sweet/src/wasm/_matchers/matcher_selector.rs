use crate::Matcher;
use anyhow::Result;
use forky_web::*;
use web_sys::*;


pub fn expect_el(selector: &str) -> Result<Matcher<HtmlElement>> {
	match Document::x_query_selector(selector) {
		Some(el) => Ok(Matcher::new(el)),
		None => {
			let result = "undefined";
			let expected = format!("body to contain selector '{selector}'");
			Err(Matcher::new(result).to_error(&expected))
		}
	}
}
