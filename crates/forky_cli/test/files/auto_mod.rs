use forky_cli::*;
use forky_core::*;
use std::{fs, path::*};
use sweet::*;

describe!("auto mod", |s| {
	s.it("works", || {
		let path = Path::new("test/files/test_dir");

		let txt = auto_mod::create_mod_text(&path.to_path_buf());
		expect(txt.as_str()).to_contain("mod _test_use;\npub use _test_use::*;")?;
		expect(txt.as_str()).to_contain("mod __test_sub_dir;\npub use __test_sub_dir::*;")?;
		expect(txt.as_str()).to_contain("pub mod test_mod;")?;

		Ok(())
	});

	s.test("double underscore", || {
		let path = Path::new("test/files/test_dir/__test_sub_dir");

		let txt = auto_mod::create_mod_text(&path.to_path_buf());
		expect(txt.as_str()).to_contain("mod test_mod;\npub use test_mod::*;")?;
		expect(txt.as_str()).to_contain("mod _test_use;\npub use _test_use::*;")?;
		Ok(())
	});
});
