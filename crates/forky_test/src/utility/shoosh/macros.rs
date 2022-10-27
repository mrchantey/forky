use inventory::{submit,*};
pub const SKIP:bool = true;

#[macro_export]
macro_rules! skip {
	() => {};
}

#[macro_export]
macro_rules! describe {
	(, $name:expr, $func:expr) => {};
	($skip:ident, $name:expr, $func:expr) => {};
	($name:expr, $func:expr) => {
		
		inventory::submit!(TestSuiteDesc {
			name: $name,
			func: $func,
			file: file!(),
		});
	};
}
