use std::path::Path;
use super::*;
use forky_core::PathExt;
use sweet::*;

pub fn suite(cases: Vec<TestCaseNative>) -> TestSuiteNative {
	TestSuiteNative {
		file: Path::new(file!()).to_forward_slash(),
		tests: cases,
		config: Default::default(),
	}
}

sweet! {
	it "works" {

		let _suite = suite(vec![
			case(TestCaseNativeFunc::Parallel(|| {
				Box::pin(async {
					panic!("hello");
				})
			})),

		]);

		let _config = TestRunnerConfig::default();

			// suite.run(&config).await;

		// expect(true).to_be_false()?;

	}
}
