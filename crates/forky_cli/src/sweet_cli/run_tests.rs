use super::*;
use anyhow::Result;
use fantoccini::Client;
use fantoccini::ClientBuilder;
use forky_core::OptionTExt;
use serde::de::DeserializeOwned;
use std::process::Command;
use std::time::Duration;
use std::time::Instant;
use sweet::*;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunTestsMode {
	Headless,
	Headed,
}


impl SweetCli {
	pub async fn run_tests(
		&self,
		should_kill: impl Fn() -> bool,
	) -> Result<Option<TestRunnerResult>> {
		let mode = self.run_tests_mode.unwrap_or(RunTestsMode::Headless);

		let mut chromedriver = Command::new("chromedriver")
			.args(["--silent", "--port=7780"])
			.spawn()?;

		let cap = if mode == RunTestsMode::Headed {
			fantoccini::wd::Capabilities::default()
		} else {
			serde_json::from_str(
				r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--headless"]}}"#,
			)
			.unwrap()
		};

		let client = ClientBuilder::native()
			.capabilities(cap)
			.connect("http://localhost:7780")
			.await
			.expect("\nCould not connect to chromedriver, is it running?\n");

		let matches = self
			.matches
			.iter()
			.map(|m| format!("m={}", m))
			.collect::<Vec<_>>()
			.join("&");

		let address = format!("{}?silent=true&{matches}", self.server.address);

		client.goto(&address).await?;

		let start_time = Instant::now();
		let mut printed_suites = 0;

		let result = loop {
			if should_kill() {
				break None;
			}
			if let Ok(results) = try_get_global::<Vec<SuiteResult>>(
				&client,
				"_sweet_result_suite",
			)
			.await
			{
				while results.len() > printed_suites {
					println!("{}", results[printed_suites].end_str());
					printed_suites += 1;
				}
			}
			if let Ok(result) = try_get_global::<TestRunnerResult>(
				&client,
				"_sweet_result_total",
			)
			.await
			{
				println!("{}", result.end_str(start_time.elapsed()));
				break Some(result);
			}
			std::thread::sleep(Duration::from_millis(10));
		};
		client.close().await?;
		chromedriver.kill()?;
		Ok(result)
	}
}

async fn try_get_global<T: DeserializeOwned>(
	c: &Client,
	prop_name: &str,
) -> Result<T> {
	let func_body = format!("return window.{}", prop_name);
	let value = c.execute(&func_body, Vec::new()).await?;
	let value = value.as_str().ok()?;
	let value = serde_json::from_str::<T>(value)?;
	Ok(value)
}
