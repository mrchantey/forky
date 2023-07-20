// use sweet::*;


// const DESC: TestSuiteDesc = TestSuiteDesc {
// 	name: "my test",
// 	func: |_| Ok(()),
// 	file: file!(),
// };
// fn setup() -> TestSuite {
// 	let mut suite = TestSuite::new(&DESC);
// 	suite.quiet = true;
// 	suite
// }

// sweet! {

// 	test "run" {
// 		let mut suite = setup();


// 		let results = suite.run(|s|{
// 			s.test("foobar", || Ok(()));
// 			Ok(())
// 		});
// 		expect(results.tests).to_be(1)?;
// 		expect(results.skipped).to_be(0)?;
// 		expect(results.failed).to_be(0)?;

// 	}

// 	test "fail" {
// 		let mut suite = setup();


// 		let results = suite.run(|s|{
// 			s.test("foobar", || {
// 				expect(true).to_be_false()?;
// 				Ok(())
// 			});
// 			Ok(())
// 		});
// 		expect(results.tests).to_be(1)?;
// 		expect(results.skipped).to_be(0)?;
// 		expect(results.failed).to_be(1)?;

// 	}
// 	test skip "skip" {
// 		let mut suite = setup();


// 		let results = suite.run(|s|{
// 			s.test("foobar", || Ok(()));
// 			Ok(())
// 	});
// 		expect(results.tests).to_be(1)?;
// 		expect(results.skipped).to_be(1)?;
// 		expect(results.failed).to_be(0)?;

// 	}


// 	// let mut a = 2;

// 	// let mut f1 = ||{
// 	// 	a = 3;
// 	// };
// 	test "scope"{
// 		// let mut suite = setup();
// 		// let f2 = ||{
// 		// 	a = 3;
// 		// };
// 		// f1();
// 		// f2();
// 		// expect(a).to_be(3);
// 		// expect(true).to_be_false()?;
// 		// fn before(){
// 		// 	a = 3;
// 		// }
// 		// suite.before(&before);
// 		// suite.before(&b);
// 	}
// }
