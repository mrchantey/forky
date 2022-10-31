use forky_core::math::*;
use std::f32::consts::{PI, TAU as FTAU};
use sweet::*;

describe!("constants", |s| {
	s.it("works", || {
		expect(TAU).to_be(FTAU)?;
		expect(HALF_TAU).to_be(PI)?;

		Ok(())
	});
});
