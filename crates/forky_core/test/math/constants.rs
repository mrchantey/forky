use forky_core::math::*;
use forky_test::*;
use std::f32::consts::{TAU as FTAU, PI};

describe!("constants", |s| {
	s.it("works", || {
		expect(TAU).to_be(FTAU)?;
		expect(HALF_TAU).to_be(PI)?;

		Ok(())
	});
});
