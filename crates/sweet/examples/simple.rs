#![feature(imported_main)]
pub use forky_test::*;
use sweet::*;

sweet! {"banana"
	let mut a = 1;
	let b = 2;
	test "pizza"{
		a = a + 1;
		expect(a).to_be(2)?;
	}
}
