use extend::ext;
use std::ffi::OsStr;

#[ext]
pub impl &str {
	/// get the first char or default
	fn first(&self) -> char { self.chars().next().unwrap_or_default() }
	/// get the last char or default
	fn last(&self) -> char { self.chars().last().unwrap_or_default() }
}

#[ext]
pub impl Option<&str> {
	fn or_default(&self) -> &str {
		match &self {
			Some(c) => c,
			None => "",
		}
	}
}
// pub impl Option<&&str> {
// 	fn or_default(&self) -> &str {
// 		match &self {
// 			Some(c) => c,
// 			None => "",
// 		}
// 	}
// }

#[ext]
pub impl Option<&OsStr> {
	fn str(&self) -> &str {
		self.unwrap_or_default().to_str().unwrap_or_default()
	}
}


#[cfg(test)]
mod test {
	use crate::prelude::*;
	use sweet::prelude::*;

	#[test]
	fn works() {
		expect("".first()).to_be('\0');
		expect("".last()).to_be('\0');
		expect("12".first()).to_be('1');
		expect("12".last()).to_be('2');
	}
}
