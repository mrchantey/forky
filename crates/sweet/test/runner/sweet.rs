use sweet::sweet;



sweet! {"banana"

	test "foobar" {
		expect(true).to_be_true()?;

	}


}
sweet! {"pizza"

	test "foobar" {
		// std::thread::sleep(std::time::Duration::from_millis(3000));
		expect(true).to_be_true()?;

	}


}
