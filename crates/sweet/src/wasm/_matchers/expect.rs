use crate::Matcher;
use anyhow::Result;
use forky_core::wasm::performance_now;
use forky_core::wasm::wait_for_millis;
use std::time::Duration;
use web_sys::HtmlElement;

pub fn expect_body() -> Matcher<HtmlElement> {
	let document = web_sys::window().unwrap().document().unwrap();
	let body = document.body().unwrap();
	Matcher::new(body)
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
