//https://www.infinyon.com/blog/2021/04/rust-custom-test-harness/


#[derive(Debug)]
pub struct TestFunc {
	pub file: &'static str,
	pub name: &'static str,
	pub func: fn(),
}

impl TestFunc {
	pub fn all_test_names() -> Vec<&'static str> {
		inventory::iter::<TestFunc>
			.into_iter()
			.map(|x| x.name)
			.collect::<Vec<&str>>()
	}

	pub fn from_name<S: AsRef<str>>(test_name: S) -> Option<&'static TestFunc> {
		inventory::iter::<TestFunc>
			.into_iter()
			.find(|t| t.name == test_name.as_ref())
	}
}

inventory::collect!(TestFunc);
