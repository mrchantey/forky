use crate::*;
// use anyhow::Result;
use futures::FutureExt;
use std::any::Any;
use tokio::task::JoinError;


type PanicResult = Result<anyhow::Result<()>, Box<dyn Any + Send>>;

pub type ResultResult = anyhow::Result<anyhow::Result<()>>;

pub fn anyhow_panic(result: PanicResult) -> ResultResult {
	match result {
		Ok(result) => Ok(result),
		Err(e) => Err(anyhow::anyhow!(panic_info(e))),
	}
}
pub fn anyhow_tokio_join(
	result: Result<anyhow::Result<()>, JoinError>,
) -> ResultResult {
	match result {
		Ok(result) => Ok(result),
		Err(e) => Err(anyhow::anyhow!(panic_info(Box::new(e)))),
	}
}

pub fn unwrap_panic(func: &fn() -> anyhow::Result<()>) -> anyhow::Result<()> {
	flatten_panic(std::panic::catch_unwind(func))
}
pub fn unwrap_panic_blocking(func: &fn() -> BoxedFuture) -> anyhow::Result<()> {
	flatten_panic(std::panic::catch_unwind(move || {
		futures::executor::block_on(func())
	}))
}

pub async fn unwrap_panic_async(
	fut: BoxedFutureUnwindSafe,
) -> anyhow::Result<()> {
	flatten_panic(fut.catch_unwind().await)
}

pub fn flatten_panic(result: PanicResult) -> anyhow::Result<()> {
	match result {
		Ok(result) => result,
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
