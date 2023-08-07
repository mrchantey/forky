use anyhow::Result;
use futures::Future;
use futures::FutureExt;
use std::any::Any;
use std::panic::UnwindSafe;
use std::pin::Pin;


pub async fn unwrap_panic_async(
	pinned_future: Pin<
		Box<dyn Send + Sync + UnwindSafe + Future<Output = Result<()>>>,
	>,
) -> Result<()> {
	match pinned_future.catch_unwind().await {
		Ok(matcher_res) => match matcher_res {
			Ok(()) => Ok(()),
			Err(err) => Err(anyhow::anyhow!(err.to_string())),
		},
		Err(e) => Err(anyhow::anyhow!(panic_info(e))),
	}
}

fn panic_info(e: Box<dyn Any + Send>) -> String {
	match e.downcast::<String>() {
		Ok(v) => *v,
		Err(e) => match e.downcast::<&str>() {
			Ok(v) => v.to_string(),
			_ => "Unknown Source of Error".to_owned(),
		},
	}
}
