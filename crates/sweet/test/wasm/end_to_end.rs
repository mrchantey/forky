use sweet::*;

sweet! {
	it e2e "works" {
		let _page = visit("https://example.com/").await?;
		// expect(true).to_be_false()?;

	}
}
