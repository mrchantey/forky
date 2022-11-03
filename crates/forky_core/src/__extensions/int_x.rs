use extend::ext;

#[ext(name = OptI32X)]
pub impl Option<&i32>{
	fn or_default(&self)->&i32{
		match &self {
			Some(c) => c,
			None => &0,
		}
	}
}