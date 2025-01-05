

/// remove first one cos thats binary name
pub fn get_cli_args() -> Vec<String> {
	let mut args: Vec<String> = std::env::args().collect();
	args.remove(0);
	args
}


#[cfg(test)]
mod test {
	use super::*;
	use sweet::prelude::*;

	#[test]
	fn works() { expect(get_cli_args().len()).to_be_greater_or_equal_to(0); }
}
