use super::*;
use crate::*;
use anyhow::Result;
use forky_core::*;
use wasm_bindgen::JsValue;
use web_sys::window;
use web_sys::HtmlIFrameElement;

pub struct TestRunnerWasm;

impl TestRunnerWasm {
	pub fn run_case(id: usize) -> Result<()> {
		let case = TestCollectorWasm::collect_case(id)?;
		wasm_bindgen_futures::spawn_local(async move {
			let window = window().unwrap();
			let parent = window.parent().unwrap().unwrap();
			let is_iframe = parent != window;
			// let origin = parent.origin();
			let origin = "*";

			match case.run().await {
				Ok(_) => {
					if is_iframe {
						parent.post_message(&JsValue::TRUE, &origin).unwrap();
					} else {
						log!("test passed");
					}
				}
				Err(err) => {
					let err = err.to_string();
					if is_iframe {
						parent.post_message(&err.into(), &origin).unwrap();
					} else {
						log!("{err}");
					}
				}
			}
		});
		// log!("{:?}", case);
		Ok(())
		//TODO get run index then
		//reflect.get(window()._sweet_wasm()._sweet_{number}).func().await
		//then send_message result to parent
		// let config = TestRunnerConfig::from_search_params();
		// TestRunnerWasm::run(&config);
	}

	pub fn run(config: &TestRunnerConfig, iframe: HtmlIFrameElement) {
		let config = config.clone();
		wasm_bindgen_futures::spawn_local(async move {
			forky_web::set_panic_hook();

			let collector = TestCollectorWasm::new();
			let logger = RunnerLoggerWasm::start(&config);

			let to_run = collector
				.suites_to_run(&config)
				.iter()
				.map(|s| {
					//TODO avoid cloning here
					let mut s = (*s).clone();
					s.iframe = Some(iframe.clone());
					s
				})
				.collect::<Vec<_>>();
			let to_run = to_run.iter().map(|s| s).collect::<Vec<_>>();

			let results = TestRunner::run_group_series::<
				SuiteLoggerDefault,
				TestCaseWasm,
			>(to_run, &config)
			.await;
			
			logger.end(&results);
		});
	}
}
