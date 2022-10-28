use std::ffi::OsStr;


// pub
pub trait StrX {
	// fn or_default(&self)->&str;
	fn first(&self)->char;
	fn last(&self)->char;
	// fn first_uppercase(&self)->bool;
	// fn first_equals(&self,other:char)->bool;
}
impl StrX for &str{
	fn first(&self)->char {
			self.chars().next().unwrap_or_default()
	}
	fn last(&self)->char {
			self.chars().last().unwrap_or_default()
	}

}
pub trait StrSliceOptionX {
	fn or_default(&self)->&str;
	// fn first_uppercase(&self)->bool;
}

impl StrSliceOptionX for Option<&str>{
	fn or_default(&self)->&str{
		match &self {
			Some(c) => c,
			None => "",
		}
	}
}
impl StrSliceOptionX for Option<&&str>{
	fn or_default(&self)->&str{
		match &self {
			Some(c) => c,
			None => "",
		}
	}
}

pub trait OsStrOptionX{
	fn str(&self)->&str;	
}
impl OsStrOptionX for Option<&OsStr>{
	fn str(&self)->&str{
		self.unwrap_or_default()
		.to_str()
		.unwrap_or_default()
	}
}