use forky_cli::*;
use std::path::*;
use sweet::*;

#[sweet_test(skip)]
fn works() -> Result<()> {
	let path = Path::new("test/files/test_dir");

	let txt = auto_mod::create_mod_text(&path.to_path_buf());
	expect(txt.as_str()).to_contain("mod test_mod;\npub use self::test_mod::*;\npub mod _test_use;\nmod __test_sub_dir;\npub use self::__test_sub_dir::*;")?;

	Ok(())
}

#[sweet_test(skip)]
fn double_underscore() -> Result<()> {
	let path = Path::new("test/files/test_dir/__test_sub_dir");

	let txt = auto_mod::create_mod_text(&path.to_path_buf());
	expect(txt.as_str()).to_contain(
		"pub mod test_mod;\nmod _test_use;\npub use self::_test_use::*;",
	)?;

	Ok(())
}
