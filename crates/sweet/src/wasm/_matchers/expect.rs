use crate::Matcher;
use anyhow::Result;
use forky_web::*;
use std::time::Duration;
use web_sys::*;

pub fn expect_body() -> Matcher<HtmlElement> {
	Matcher::new(Document::x_body())
}

pub async fn poll_ok(f: impl Fn() -> Result<()>) -> Result<()> {
	poll_ok_with_timeout(f, Duration::from_secs(4)).await
}

pub async fn poll_ok_with_timeout(
	f: impl Fn() -> Result<()>,
	timeout: Duration,
) -> Result<()> {
	let start = performance_now();
	loop {
		match f() {
			Ok(()) => return Ok(()),
			Err(err) => {
				if performance_now() - start > timeout.as_millis() as f64 {
					return Err(err);
				}
				wait_for_millis(10).await;
			}
		}
	}
}
