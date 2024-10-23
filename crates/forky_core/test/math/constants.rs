use forky_core::math::*;
use std::f32::consts::PI;
use std::f32::consts::TAU as FTAU;
use sweet::*;

#[sweet_test]
fn works() -> Result<()> {
	expect(TAU).to_be(FTAU)?;
	expect(HALF_TAU).to_be(PI)?;

	Ok(())
}
