use sweet::*;


sweet! {

	it "works" {
		let ctx = backtracer::file_context();
		expect(ctx.as_str()).to_contain("let ctx = backtracer::file_context();")?;
		// expect(false).to_be_true()?;
	}

}
