#![feature(imported_main)]
pub use sweet::*;

sweet! {

	it "passes"{
		expect(true).to_be_true()?;
	}
	it skip "fails"{
		expect(true).to_be_false()?;
	}
}
