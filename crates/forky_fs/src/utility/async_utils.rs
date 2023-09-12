use std::future::Future;
use tokio::time::sleep;
use tokio::time::Duration;

pub async fn retry_async<T, E, F>(
	func: impl Fn() -> F,
	timeout: Duration,
) -> Result<T, E>
where
	F: Future<Output = Result<T, E>>,
{
	let start = std::time::Instant::now();
	loop {
		match func().await {
			Ok(val) => return Ok(val),
			Err(err) => {
				if start.elapsed() > timeout {
					return Err(err);
				}
			}
		}
		sleep(Duration::from_millis(10)).await;
	}
}
