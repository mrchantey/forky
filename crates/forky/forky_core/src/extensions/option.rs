use anyhow::*;
use extend::ext;

#[ext]
pub impl<T> Option<T> {
	fn ok(self) -> Result<T> { option_to_result(self) }
}

pub fn option_to_result<T>(option: Option<T>) -> Result<T> {
	match option {
		Some(value) => Ok(value),
		None => Err(anyhow!("Expected Some")),
	}
}

#[ext]
pub impl Option<bool> {
	fn is_true(&self) -> bool {
		if let Some(value) = self {
			*value
		} else {
			false
		}
	}
	fn is_none_or_false(&self) -> bool {
		if let Some(value) = self {
			!value
		} else {
			true
		}
	}
}
