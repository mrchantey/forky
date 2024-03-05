use forky_web::future_timeout;
use forky_web::wait_for_millis;
use std::time::Duration;
use sweet::*;

#[sweet_test]
pub async fn works() -> Result<()> {

	let a = future_timeout(
		|| async {
			wait_for_millis(400).await;
			39
		},
		Duration::from_millis(500),
	)
	.await?;
	expect(a).to_be(39)?;

	let err = future_timeout(
		|| async {
			wait_for_millis(600).await;
		},
		Duration::from_millis(500),
	)
	.await;
	expect(err).to_be_err_str("Timeout")?;

	Ok(())
}
