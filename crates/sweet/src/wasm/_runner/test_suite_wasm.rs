use crate::*;
use anyhow::Ok;
use forky_web::*;
use web_sys::HtmlIFrameElement;
use web_sys::MessageEvent;
use web_sys::UrlSearchParams;

#[derive(Default, Debug, Clone)]
pub struct TestSuiteWasm {
	pub file: String,
	pub tests: Vec<TestCaseWasm>,
	pub config: TestSuiteConfig,
	pub iframe: Option<HtmlIFrameElement>,
}

impl TestSuiteTrait<TestCaseWasm> for TestSuiteWasm {
	fn new(file: String) -> Self {
		Self {
			file,
			tests: Vec::new(),
			config: TestSuiteConfig::default(),
			iframe: None,
		}
	}
	fn file(&self) -> &str { self.file.as_str() }
	fn config(&self) -> &TestSuiteConfig { &self.config }
	fn tests(&self) -> &Vec<TestCaseWasm> { &self.tests }
	fn tests_mut(&mut self) -> &mut Vec<TestCaseWasm> { &mut self.tests }

	async fn run_cases(
		&self,
		to_run: Vec<&TestCaseWasm>,
		_runner_config: &TestRunnerConfig,
	) -> Vec<anyhow::Error> {
		let mut results = Vec::with_capacity(to_run.len());
		if let Some(iframe) = &self.iframe {
			for case in to_run {
				#[rustfmt::skip]
			let is_e2e = self.config.cases.context == TestRunEnvironment::EndToEnd
				|| case.config.context == TestRunEnvironment::EndToEnd;

				let result = if is_e2e {
					iframe.x_reload();
					//TODO await reload
					// iframe.x_reload_async().await;
					case.run().await
				} else {
					run_case_unit(&iframe, case).await
				};
				if let Err(result) = result {
					results.push(result);
				}
			}
		} else {
			panic!("iframe not set");
		}
		results
	}
}

async fn run_case_unit(
	iframe: &HtmlIFrameElement,
	case: &TestCaseWasm,
) -> anyhow::Result<()> {
	let params = UrlSearchParams::new().unwrap();
	params.set("testid", &case.id.to_string());
	let mut url = params.to_string().as_string().unwrap();
	url.insert_str(0, "?");
	iframe.set_src(&url);
	let ev = HtmlEventListener::wait("message").await;
	let ev: MessageEvent = ev.into();
	let data = ev.data();
	if data.is_string() {
		Err(anyhow::anyhow!(data.as_string().unwrap()))
	} else {
		Ok(())
	}
}
