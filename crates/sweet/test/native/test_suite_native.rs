use super::*;
use sweet::*;

pub fn suite(cases: Vec<TestCaseNative>) -> TestSuiteNative {
	TestSuiteNative {
		file: "some/path.rs".to_string(),
		tests: cases,
		config: Default::default(),
	}
}

sweet! {
	it "works" {

		let _suite = suite(vec![
			case(|| {
				Box::pin(async {
					panic!("hello");
				})
			}),

		]);

		let _config = TestRunnerConfig::default();

			// suite.run(&config).await;

		// expect(true).to_be_false()?;

	}
}
