use sweet::*;

sweet! {

	test "foobar" {
		// std::thread::sleep(std::time::Duration::from_millis(3000));
		expect(true).to_be_true()?;
	}
	it "works async" {
		async fn foobar(){}
		foobar().await;

		// println!("foobar");
		expect(true).to_be_true()?;
	}
}
