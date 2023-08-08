use crate::*;
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
impl TestSuiteWasm {
	async fn run_cases_unit(
		&self,
		to_run: Vec<&TestCaseWasm>,
	) -> Vec<anyhow::Error> {
		if let Some(iframe) = &self.iframe {
			let mut results = Vec::with_capacity(to_run.len());
			for case in to_run {
				let params = UrlSearchParams::new().unwrap();
				params.set("testid", &case.id.to_string());
				let mut url = params.to_string().as_string().unwrap();
				url.insert_str(0, "?");
				iframe.set_src(&url);
				let ev = HtmlEventListener::wait("message").await;
				let ev: MessageEvent = ev.into();
				let data = ev.data();
				if data.is_string() {
					results.push(anyhow::anyhow!(data.as_string().unwrap()));
				}
			}
			results
		} else {
			panic!("iframe not set");
		}
	}
	async fn run_cases_e2e(
		&self,
		to_run: Vec<&TestCaseWasm>,
	) -> Vec<anyhow::Error> {
		if let Some(iframe) = &self.iframe {
			run_cases_series_with_before(to_run, async move |_| {
				iframe.x_reload();
			})
			.await
		} else {
			panic!("iframe not set");
		}
	}
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
		match self.config.context {
			TestSuiteContext::Unit => self.run_cases_unit(to_run).await,
			TestSuiteContext::EndToEnd => self.run_cases_e2e(to_run).await,
		}
		// if self.config.
		// if config.parallel {
		// 	run_cases_parallel(to_run, config).await
		// } else {
		// }
	}
}
