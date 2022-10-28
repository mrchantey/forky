use forky_test::*;
use forky_core::*;

describe!("int", |s| {

	s.it("works", || {
		let a = 1;
		let _a = Some(&a);
		expect(*_a.unwrap()).to_be(1)?;
		expect(*_a.or_default()).to_be(1)?;
		let _a:Option<&i32> = None;
		expect(*_a.or_default()).to_be(0)?;
		
		Ok(())
	});
});