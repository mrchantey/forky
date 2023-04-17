use sweet::*;

sweet! {
	test "to_be_bool" {
		//lmao
		let result = expect(true).to_be_true();
		expect(result.is_ok()).to_be(true)?;

		let result = expect(true).to_be_false();
		expect(result.is_ok()).to_be(false)?;
		let result = expect(true).to_be(false);
		expect(result.is_ok()).to_be(false)?;
		expect(result.unwrap_err().to_string().as_str()).to_contain("this line")?;

	}

	test "to_be" {
		//lmao
		let result = expect(true).to_be(true);
		expect(result.is_ok()).to_be(true)?;

		let result = expect(true).to_be(false);
		expect(result.unwrap_err().to_string().as_str()).to_contain("this line")?;

	}

	test "to_contain" {
		let result = expect("foo").to_contain("foo");
		expect(result.is_ok()).to_be_true()?;
		let result = expect("foo").to_contain("bar");
		expect(result.is_err()).to_be_true()?;
	}

	test "to_be_close_to"{
		let result = expect(0.).to_be_close_to(0.);
		expect(result.is_ok()).to_be_true()?;
		let result = expect(-0.999).to_be_close_to(-1.);
		expect(result.is_ok()).to_be_true()?;
		let result = expect(0.9).to_be_close_to(1.01);
		expect(result.is_ok()).to_be_false()?;
	}
	test "order"{
		let result = expect(0).to_be_greater_or_equal_to(0);
		expect(result.is_ok()).to_be_true()?;
		let result = expect(10).to_be_greater_than(-10);
		expect(result.is_ok()).to_be_true()?;
		let result = expect(10).to_be_greater_than(11);
		expect(result.is_ok()).to_be_false()?;
	}
}
