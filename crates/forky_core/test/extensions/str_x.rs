use forky_core::prelude::*;
use sweet::*;

#[sweet_test]
fn works() -> Result<()> {
	expect("".first()).to_be('\0')?;
	expect("".last()).to_be('\0')?;
	expect("12".first()).to_be('1')?;
	expect("12".last()).to_be('2')?;

	Ok(())
}
