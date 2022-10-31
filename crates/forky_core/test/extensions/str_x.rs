use forky_core::*;
use sweet::*;

sweet! {
	it "works" {
		expect("".first()).to_be('\0')?;
		expect("".last()).to_be('\0')?;
		expect("12".first()).to_be('1')?;
		expect("12".last()).to_be('2')?;
	}
}