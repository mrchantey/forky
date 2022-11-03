use sweet::*;

sweet! {
	it "works" {
		// expect(true).to_be(false)?;
		let b:&dyn FnMut()->();
		let mut a = 2;
		let func = ||{
			a = 3;
		};
		let mut action = Action::new(func);

		action.run();
		expect(a).to_be(3)?;
	}
}
