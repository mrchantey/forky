#[macro_export]
macro_rules! test {
	($name:expr, $func:expr) => {
		inventory::submit!(TestFunc {
			name: $name,
			func: $func,
			file: file!(),
			// func_type:TestFuncType::Test,
		});
	};
}

#[macro_export]
macro_rules! describe {
	($name:expr, $func:expr) => {
		inventory::submit!(TestSuiteDesc {
			name: $name,
			func: $func,
			file: file!(),
		});
	};
}
