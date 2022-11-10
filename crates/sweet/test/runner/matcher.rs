use forky_core::log;
use sweet::*;

sweet! {
	test "to_be_bool" {
		//lmao
		let result = expect(true).to_be_true();
		expect(result.is_ok()).to_be(true)?;

		let result = expect(true).to_be_false();
		let result = expect(true).to_be(false);
		expect(result.is_ok()).to_be(false)?;
		expect(result.unwrap_err().message.as_str()).to_contain("this line")?;

	}

	test "to_be" {
		//lmao
		let result = expect(true).to_be(true);
		expect(result.is_ok()).to_be(true)?;

		let result = expect(true).to_be(false);
		expect(result.unwrap_err().message.as_str()).to_contain("this line")?;

	}

	test "to_contain" {
		let result = expect("foo").to_contain("foo");
		expect(result.is_ok()).to_be_true()?;
		let result = expect("foo").to_contain("bar");
		expect(result.unwrap_err().message.as_str()).to_contain("this line")?;
	}

	test "to_be_close_to"{
		let result = expect(0.).to_be_close_to(0.);
		expect(result.is_ok()).to_be_true()?;
		let result = expect(-0.999).to_be_close_to(-1.);
		expect(result.is_ok()).to_be_true()?;
		let result = expect(0.9).to_be_close_to(1.);
		expect(result.is_ok()).to_be_false()?;
	}
	test "to_be_at_least"{
		let result = expect(0).to_be_at_least(0);
		expect(result.is_ok()).to_be_true()?;
		let result = expect(10).to_be_at_least(-10);
		expect(result.is_ok()).to_be_true()?;
		let result = expect(10).to_be_at_least(11);
		expect(result.is_ok()).to_be_false()?;
	}
}
