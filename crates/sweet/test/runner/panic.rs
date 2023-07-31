use sweet::*;


// how wasm-bindgen-test does it
//https://github.com/rustwasm/wasm-bindgen/blob/74bfc1f85ead6a3e0c37a86e5f93df3e692e217a/crates/test/src/rt/mod.rs#L227-L240


sweet! {

	it "handles panics"{
		std::panic::set_hook(Box::new(|panic_info| {
			println!("hello!");

		}));
		// std::thread::sleep(std::time::Duration::from_secs(2));
		assert!(true ==false);
	}
	it skip "fails"{
		expect(true).to_be_false()?;
	}
}
