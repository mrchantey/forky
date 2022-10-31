use forky_core::*;
use sweet::*;


const DESC: TestSuiteDesc = TestSuiteDesc {
	name: "my test",
	func: |_| {},
	file: file!(),
};

sweet!{
	fn setup() -> TestSuite {
		let mut suite = TestSuite::new(&DESC);
		suite.quiet = true;
		suite
	}

	test "run" {
		let mut suite = setup();

		suite.test("foobar", || Ok(()));

		let results = suite.results();
		expect(results.tests).to_be(1)?;
		expect(results.skipped).to_be(0)?;
		expect(results.failed).to_be(0)?;

	}

	test "fail" {
		let mut suite = setup();

		suite.test("foobar", || {
			expect(true).to_be_false()?;
			Ok(())
		});

		let results = suite.results();
		expect(results.tests).to_be(1)?;
		expect(results.skipped).to_be(0)?;
		expect(results.failed).to_be(1)?;

	}
	test skip "skip" {
		let mut suite = setup();

		suite.test("foobar", || Ok(()));

		let results = suite.results();
		expect(results.tests).to_be(1)?;
		expect(results.skipped).to_be(1)?;
		expect(results.failed).to_be(0)?;

	}
}
