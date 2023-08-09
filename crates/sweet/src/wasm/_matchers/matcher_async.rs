use crate::*;
use anyhow::Result;
use forky_web::*;
use std::time::Duration;


impl<T> Matcher<T> {
	pub async fn poll<T2>(
		&self,
		func: impl Fn(&Self) -> Result<T2> + 'static,
	) -> Result<T2> {
		poll_ok(|| func(self)).await
		// match self {
		// 	Matcher::AsyncMatcher(m) => m.poll().await,
		// 	Matcher::SyncMatcher(m) => Ok(m.clone()),
		// }
	}
}

// pub trait IntoAsyncMatcher<T>
// where
// 	Self: MatcherTrait<T>,
// {
// 	async fn poll(&self, func: ) -> Result<T>;
// }


// impl<T, F> IntoAsyncMatcher<T> for F
// where
// 	F: Fn() -> Result<T> + 'static,
// {
// 	async fn poll(&self) -> Result<T> { poll_ok(self).await }
// }


async fn poll_ok<T>(f: impl Fn() -> Result<T>) -> Result<T> {
	poll_ok_with_timeout(f, Duration::from_secs(2)).await
}

async fn poll_ok_with_timeout<T>(
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
