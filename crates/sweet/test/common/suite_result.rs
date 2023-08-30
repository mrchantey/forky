use forky_core::*;
use sweet::*;

sweet! {
	it "works" {
		let file = std::path::Path::new(file!()).to_forward_slash();
		let result = SuiteResult::new(file, 0, 0);
		let end = result.end_str();
		expect(end.as_str()).to_be("\u{1b}[42;1;30m PASS \u{1b}[0;39;49m \u{1b}[2mcrates/sweet/test/common\u{1b}[0;39;49m\u{1b}[2m/\u{1b}[0;39;49m\u{1b}[1msuite_result.rs\u{1b}[0;39;49m")?;
	}
}