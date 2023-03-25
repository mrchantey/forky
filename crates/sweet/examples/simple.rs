#![feature(imported_main)]
pub use sweet::*;

sweet! {"banana"
	let mut a = 1;

	before{
		a = 0;
	}

	after{
		// a = 10;
	}

	test "pizza"{
		a = a + 1;
		expect(a).to_be(2)?;
		// let a = 2;
		// let b = "hi";
		// let c = a + b;
	}
	test "taco"{
		a = a + 1;
		expect(a).to_be(2)?;
	}
}
