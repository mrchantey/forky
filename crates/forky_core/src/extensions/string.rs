use extend::ext;

#[ext]
pub impl String {
	fn push_string(&mut self, str: &String) -> &Self {
		self.push_str(str.as_str());
		self
	}
	fn push_str_line(&mut self, str: &str) -> &Self {
		self.push_str(str);
		self.push('\n');
		self
	}
}