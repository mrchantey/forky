#![cfg_attr(test, feature(test, custom_test_frameworks))]
#![cfg_attr(test, test_runner(sweet::test_runner))]

#[cfg(test)]
mod test {
	use ::forky_cli::prelude::*;
	use std::path::*;
	use sweet::prelude::*;

	#[test]
	fn works() {
		const EXPECTED: &str = "pub mod included_dir;\npub mod included_file;\n#[allow(unused_imports)]\npub use self::included_file::*;\n";

		let path = Path::new("tests/test_dir");

		let txt = AutoModCommand::default()
			.create_mod_text(&path.to_path_buf())
			.unwrap();
		// let path = Path::new("crates/forky_cli/tests/test_dir");

		expect(txt.as_str()).to_be(EXPECTED);
	}
}


// #[test]
// fn works() {
// }
