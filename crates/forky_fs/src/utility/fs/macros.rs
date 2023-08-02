#[macro_export]
macro_rules! file_abs {
	() => {
		std::path::Path::new(env!("CARGO_MANIFEST_DIR").join(file!()))
	};
}

#[macro_export]
macro_rules! file_abs_workspace {
	() => {
		std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
			.join(forky_fs::fs::pop_first_two_path_components(file!()))
	};
}
