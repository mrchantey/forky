use anyhow::Result;
use fantoccini::Client;
use fantoccini::ClientBuilder;
use forky_core::OptionTExt;
use serde::de;
use std::time::Duration;
use std::time::Instant;
// use std::time::Instant;
use sweet::*;



#[tokio::main]
pub async fn main() -> Result<()> {
	let cap: fantoccini::wd::Capabilities = serde_json::from_str(
		r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--headless"]}}"#,
	)
	.unwrap();

	let c = ClientBuilder::native()
		.capabilities(cap)
		.connect("http://localhost:9515")
		.await?;
	c.goto("http://localhost:7777").await?;

	let start_time = Instant::now();
	let mut printed_suites = 0;
	loop {
		if let Ok(results) =
			try_get_global::<Vec<SuiteResult>>(&c, "_sweet_result_suite").await
		{
			while results.len() > printed_suites {
				println!("{}", results[printed_suites].end_str());
				printed_suites += 1;
			}
		}
		if let Ok(result) =
			try_get_global::<TestRunnerResult>(&c, "_sweet_result_total").await
		{
			println!("{}", result.end_str(start_time.elapsed()));
			break;
		}
		std::thread::sleep(Duration::from_millis(10));
	}
	c.close().await?;
	Ok(())
}



async fn try_get_global<T: de::DeserializeOwned>(
	c: &Client,
	prop_name: &str,
) -> Result<T> {
	let func_body = format!("return window.{}", prop_name);
	let value = c.execute(&func_body, Vec::new()).await?;
	let value = value.as_str().ok()?;
	let value = serde_json::from_str::<T>(value)?;
	Ok(value)
}
