use extend::ext;
use std::ffi::OsStr;

#[ext]
pub impl &str {
	fn first(&self) -> char { self.chars().next().unwrap_or_default() }
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
