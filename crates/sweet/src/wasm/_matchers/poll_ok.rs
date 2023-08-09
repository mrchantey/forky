use anyhow::Result;
use forky_web::*;
use std::time::Duration;

pub trait IntoAsyncMatcher<T>
where
	Self: Sized,
{
	async fn poll(&self) -> Result<T>;
}


impl<T, F> IntoAsyncMatcher<T> for F
where
	F: Fn() -> Result<T> + 'static,
{
	async fn poll(&self) -> Result<T> { poll_ok(self).await }
}


pub async fn poll_ok<T>(f: impl Fn() -> Result<T>) -> Result<T> {
	poll_ok_with_timeout(f, Duration::from_secs(4)).await
}

pub async fn poll_ok_with_timeout<T>(
	f: impl Fn() -> Result<T>,
	timeout: Duration,
) -> Result<T> {
	let start = performance_now();
	loop {
		match f() {
			Ok(val) => return Ok(val),
			Err(err) => {
				if performance_now() - start > timeout.as_millis() as f64 {
					return Err(err);
				}
				wait_for_millis(10).await;
			}
		}
	}
}
