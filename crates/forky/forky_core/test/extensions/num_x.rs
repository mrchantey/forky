use forky_core::*;
use sweet::*;

#[sweet_test]
pub fn i32() -> Result<()> {
	let a = 1;
	let _a = Some(&a);
	expect(*_a.unwrap()).to_be(1)?;
	expect(*_a.or_default()).to_be(1)?;
	let _a: Option<&i32> = None;
	expect(*_a.or_default()).to_be(0)?;

	Ok(())
}



#[sweet_test]
pub fn works() -> Result<()> {
	expect(f32::lerp(1., 2., 0.5)).to_be(1.5)?;
	Ok(())
}
