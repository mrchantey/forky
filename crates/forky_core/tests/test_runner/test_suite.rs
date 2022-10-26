use forky_core::testing::*;
use forky_core::*;


const DESC: TestSuiteDesc = TestSuiteDesc {
	name: "my test",
	func: |_| {},
	file: file!(),
};

describe!("test suite", |s| {
	fn setup() -> TestSuite {
		let mut suite = TestSuite::new(&DESC);
		suite.quiet = true;
		suite
	}

	s.test("run", || {
		let mut suite = setup();

		suite.test("foobar", || Ok(()));

		let results = suite.results();
		expect(results.tests).to_be(1)?;
		expect(results.skipped).to_be(0)?;
		expect(results.failed).to_be(0)?;

		Ok(())
	});
	s.test("fail", || {
		let mut suite = setup();

		suite.test("foobar", || {
			expect(true).to_be_false()?;
			Ok(())
		});

		let results = suite.results();
		expect(results.tests).to_be(1)?;
		expect(results.skipped).to_be(0)?;
		expect(results.failed).to_be(1)?;

		Ok(())
	});
	s.test("skip", || {
		let mut suite = setup();

		suite.skip().test("foobar", || Ok(()));

		let results = suite.results();
		expect(results.tests).to_be(1)?;
		expect(results.skipped).to_be(1)?;
		expect(results.failed).to_be(0)?;

		Ok(())
	});
});


//slow
fn long_fn() -> f32 {
	let mut a = 3290.;
	for _x in 0..100000 {
		for _y in 0..10000 {
			a = f32::sqrt(a);
		}
	}
	a
}

describe!(skip,"slow test", |s| {
	s.it("works slowly", || {
		let a = long_fn();
		expect(a).not().to_be(0.)?;
		Ok(())
	});
});
