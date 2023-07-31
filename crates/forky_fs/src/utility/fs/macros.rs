use std::path::Path;
use std::path::PathBuf;

#[macro_export]
macro_rules! file_abs {
	() => {
		Path::new(env!("CARGO_MANIFEST_DIR").join(file!()))
	};
}

#[macro_export]
macro_rules! file_abs_workspace {
	() => {
		Path::new(env!("CARGO_MANIFEST_DIR"))
			.join(forky_fs::fs::pop_first_two_path_components(file!()))
	};
}
