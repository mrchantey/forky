#![feature(imported_main)]
pub use sweet::*;

sweet! {

	it "passes"{
		expect("foobar").not().to_start_with("foo")?;
	}


	it skip "fails"{
		expect(true).to_be_false()?;
	}
}
