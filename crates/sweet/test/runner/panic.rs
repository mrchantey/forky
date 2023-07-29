use sweet::*;

sweet! {

	it skip "handles panics"{
		// std::thread::sleep(std::time::Duration::from_secs(2));
		assert!(true ==false);
	}
	it skip "fails"{
		expect(true).to_be_false()?;
	}
}
