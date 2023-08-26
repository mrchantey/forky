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


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunMode {
	Interactive,
	Headless,
	Headed,
}


impl SweetCli {
	pub async fn handle_run_mode(
		&self,
		should_kill: impl Fn() -> bool,
	) -> Result<()> {
		if self.run_mode == RunMode::Interactive {
			return Ok(());
		}

		let mut chromedriver = Command::new("chromedriver")
			.args(["--silent", "--port=9515"])
			.spawn()?;

		let cap = if self.run_mode == RunMode::Headed {
			fantoccini::wd::Capabilities::default()
		} else {
			serde_json::from_str(
				r#"{"browserName":"chrome","goog:chromeOptions":{"args":["--headless"]}}"#,
			)
			.unwrap()
		};

		let client = ClientBuilder::native()
			.capabilities(cap)
			.connect("http://localhost:9515")
			.await
			.expect("\nCould not connect to chromedriver, is it running?\n");

		client.goto(&self.server.address.to_string()).await?;

		let start_time = Instant::now();
		let mut printed_suites = 0;

		loop {
			if should_kill() {
				break;
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
				break;
			}
			std::thread::sleep(Duration::from_millis(10));
		}
		client.close().await?;
		chromedriver.kill()?;
		Ok(())
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
