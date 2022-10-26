
pub trait StrX {
	fn or_default(&self)->&str;
}

impl StrX for Option<&str>{
	fn or_default(&self)->&str{
		match &self {
			Some(c) => c,
			None => "",
		}
	}
}
impl StrX for Option<&&str>{
	fn or_default(&self)->&str{
		match &self {
			Some(c) => c,
			None => "",
		}
	}
}