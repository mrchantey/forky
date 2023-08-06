// use anyhow::Result;
// use futures::Future;
use sweet::*;


fn case(func: TestCaseNativeFn) -> TestCaseNative {
	TestCaseNative {
		file: "some/path.rs",
		name: "works",
		func,
		config: TestCaseConfig::Default,
	}
}


sweet! {
	it "works" {
		let case = case(|| {
			Box::pin(async {
				panic!("hello");
			})
		});
		expect(case.run_func().await).to_be_err_str("hello")?;
	}
}
