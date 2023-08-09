use crate::*;
use anyhow::Result;
use forky_web::*;

impl<T> Matcher<T> {
	pub async fn poll<T2>(
		&self,
		func: impl Fn(&Self) -> Result<T2> + 'static,
	) -> Result<T2> {
		poll_ok(|| func(self)).await
	}
}