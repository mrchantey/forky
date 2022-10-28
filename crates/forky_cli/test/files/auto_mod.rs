use std::{
	fs,
	path::*,
};
use forky_cli::*;
use forky_core::*;
use forky_test::*;

describe!("auto mod", |s| {
	s.it("works", || {
		let path = Path::new("test/files");

		let foo = auto_mod::create_mod_text(&path.to_path_buf());
		expect(foo.as_str().contains("mod auto_mod;\npub use auto_mod::*;")).to_be_true()?;

		Ok(())
	});
});
