use super::*;
use sweet::*;

pub fn suite(cases: Vec<TestCaseNative>) -> TestSuiteNative {
	TestSuiteNative {
		file: String::from("some/path.rs"),
		// name: "works",
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
