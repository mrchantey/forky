use crate::BoxedFutureUnwindSafe;
use anyhow::Result;
use futures::FutureExt;
use std::any::Any;

pub async fn unwrap_panic_async(fut: BoxedFutureUnwindSafe) -> Result<()> {
	match fut.catch_unwind().await {
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
