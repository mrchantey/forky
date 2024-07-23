use forky_core::math::*;
use std::f32::consts::{PI, TAU as FTAU};
use sweet::*;

sweet! {
	it "works"{
		expect(TAU).to_be(FTAU)?;
		expect(HALF_TAU).to_be(PI)?;
	}
}
