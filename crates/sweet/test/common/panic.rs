use sweet::*;


// how wasm-bindgen-test unwinds panics
//https://github.com/rustwasm/wasm-bindgen/blob/74bfc1f85ead6a3e0c37a86e5f93df3e692e217a/crates/test/src/rt/mod.rs#L227-L240

sweet! {

	it skip "handles panics"{
		assert!(true ==false);
				// panic!("YESSS");
	}
	it skip "fails"{
		// expect(true).to_be_false()?;
		expect(true).to_be_false()?;
	}
}
