// use anyhow::Result;
// use futures::Future;
use sweet::native::*;
use sweet::test_case::*;



pub fn case(func: TestCaseNativeFunc) -> TestCaseNative {
	TestCaseNative {
		file: "some/path.rs",
		name: "works",
		func,
		config: TestCaseConfig::default(),
	}
}


// sweet! {
// 	test "panic" {
// 		let case = case(|| {
// 			Box::pin(async {
// 				panic!("hello");
// 			})
// 		});
// 		expect(case.run_func().await).to_be_err_str("hello")?;
// 	}
// 	test "filename"{
// 		let case = case(|| {Box::pin(async {Ok(())	})});
// 		expect(case.path().to_str().unwrap()).to_be(&"some/path.rs")?;
// 	}
// }
