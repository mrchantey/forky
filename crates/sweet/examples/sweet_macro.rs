#![feature(imported_main)]
pub use sweet::*;

async fn foo() {}

sweet! {
	it{}
	it{foo().await;}
	it nonSend{foo().await;}
	// it "passes"{
	// 	expect("foobar").not().to_start_with("foo")?;
	// }


	// it skip "fails"{
	// 	expect(true).to_be_false()?;
	// }
}
