
pub trait IntX {
	fn or_default(&self)->&i32;
}

impl IntX for Option<&i32>{
	fn or_default(&self)->&i32{
		match &self {
			Some(c) => c,
			None => &0,
		}
	}
}