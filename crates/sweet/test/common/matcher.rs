use anyhow::*;
use sweet::*;


// fn foo() {}
#[derive(Clone)]
struct NewType<T>(pub T);

impl<T> std::ops::Deref for NewType<T> {
	type Target = T;
	fn deref(&self) -> &Self::Target { &self.0 }
}

sweet! {

	test "equality" {
		expect(true).to_be(true)?;
		expect(true).not().to_be(false)?;
	}

	test "bool" {
		expect(true).to_be_true()?;
		expect(false).not().to_be_true()?;

		expect(false).to_be_false()?;
		expect(true).not().to_be_false()?;
	}


	test "to_be_close_to"{
		expect(0.).to_be_close_to(0.)?;
		expect(-0.999).to_be_close_to(-1.)?;
		expect(0.9).not().to_be_close_to(1.01)?;
		expect(&NewType(0.0_f64)).to_be_close_to(0.)?;

		expect(0.0_f32).to_be_close_to(0.)?;
		expect(&NewType(0.0_f32)).to_be_close_to(0.)?;
	}
	test "order"{
		expect(0).to_be_greater_or_equal_to(0)?;
		expect(10).to_be_greater_than(-10)?;
		expect(10).not().to_be_greater_than(11)?;
	}

	test "str"{
		// expect("foo".to_string()).to_be("foo")?;

		expect("foobar").to_contain("bar")?;
		expect("foobar").not().to_contain("baz")?;

		expect("foobar").to_start_with("foo")?;
		expect("foobar").not().to_start_with("bar")?;

		expect("foobar").to_end_with("bar")?;
		expect("foobar").not().to_end_with("foo")?;

	}

	test "option"{
		expect(Some(true)).to_be_some()?;
		expect(Some(true)).not().to_be_none()?;

		expect(None::<bool>).to_be_none()?;
		expect(None::<bool>).not().to_be_some()?;
	}

	test "result"{
		let ok = || -> anyhow::Result<()> { Ok(()) };
		expect(ok()).to_be_ok()?;
		expect(ok()).not().to_be_err()?;

		let err = || -> anyhow::Result<()> { Err(anyhow!("foo")) };

		expect(err()).to_be_err()?;
		expect(err()).not().to_be_ok()?;

		expect(err()).to_be_err_str("foo")?;
		expect(err()).not().to_be_err_str("foobar")?;
	}

	test "mock func"{
		let func = mock_trigger();
		func.call(());
		func.call(());
		expect(&func).to_have_been_called()?;
		expect(&func).to_have_been_called_times(2)?;
		expect(&func.clone()).not().to_have_been_called_times(1)?;
	}
	test "mock trigger"{
		let func = mock_trigger();
		func.call0();
		func.call0();
		expect(&func).to_have_been_called()?;
		expect(&func).to_have_been_called_times(2)?;
		expect(&func.clone()).not().to_have_been_called_times(1)?;
	}
	test "mock func"{
		let func = mock_func(|i|i * 2);
		func.call(0);
		func.call(2);
		expect(&func).to_have_been_called()?;
		expect(&func).to_have_returned_with(&0)?;
		expect(&func).not().to_have_returned_with(&4)?;
		expect(&func).nth_return(1)?.to_be(4)?;
		expect(&func).nth_return(0)?.to_be(0)?;
		expect(&func).nth_return(1)?.to_be(4)?;
	}
}
