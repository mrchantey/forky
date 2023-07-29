use sweet::*;

sweet! {

	it "handles panics"{
		// std::thread::sleep(std::time::Duration::from_secs(2));
		// expect(true).to_be_true()?;
		assert!(true ==false);
		// panic!("this should panic");
	}
}
