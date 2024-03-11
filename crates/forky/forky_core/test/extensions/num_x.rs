use forky_core::*;
use sweet::*;


#[sweet_test]
pub fn f32() -> Result<()> {
	expect(f32::lerp(1., 2., 0.5)).to_be(1.5)?;
	Ok(())
}
