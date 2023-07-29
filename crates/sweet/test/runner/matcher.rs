use sweet::*;

//TODO test negated for all matchers

sweet! {

	test "to_be" {
		//lol this doesnt actually assert anything
		let result = expect(true).to_be(true);
		expect(result.is_ok()).to_be(true)?;
		let result = expect(true).not().to_be(true);
		expect(result.is_ok()).to_be(false)?;
	}

	test "to_be_close_to"{
		let result = expect(0.).to_be_close_to(0.);
		expect(result.is_ok()).to_be_true()?;
		let result = expect(-0.999).to_be_close_to(-1.);
		expect(result.is_ok()).to_be_true()?;
		let result = expect(0.9).to_be_close_to(1.01);
		expect(result.is_ok()).to_be_false()?;
		let result = expect(0.9).not().to_be_close_to(1.01);
		expect(result.is_ok()).to_be_true()?;
	}
	test "order"{
		let result = expect(0).to_be_greater_or_equal_to(0);
		expect(result.is_ok()).to_be_true()?;
		let result = expect(10).to_be_greater_than(-10);
		expect(result.is_ok()).to_be_true()?;
		let result = expect(10).to_be_greater_than(11);
		expect(result.is_ok()).to_be_false()?;

		let result = expect(10).not().to_be_greater_than(11);
		expect(result.is_ok()).to_be_true()?;
	}

	test "str"{
		let result = expect("foobar").to_contain("bar");
		expect(result.is_ok()).to_be_true()?;
		let result = expect("foobar").to_contain("baz");
		expect(result.is_ok()).to_be_false()?;
		let result = expect("foobar").not().to_contain("bar");
		expect(result.is_ok()).to_be_false()?;
		let result = expect("foobar").not().to_contain("baz");
		expect(result.is_ok()).to_be_true()?;
	}
	test "bool" {
		let result = expect(true).to_be_true();
		expect(result.is_ok()).to_be(true)?;

		let result = expect(true).to_be_false();
		expect(result.is_ok()).to_be(false)?;
		let result = expect(true).not().to_be(false);
		expect(result.is_ok()).to_be(true)?;

	}
}
