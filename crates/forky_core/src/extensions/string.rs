pub trait StringX {
	// fn to_str<P: AsRef<Path>>(parent: &PathBuf, path: P) -> Self;
	//TODO rename slice
	fn to_str(&self) -> &str;
	fn push_str_line(&mut self, str: &str) -> &Self;
	fn push_string(&mut self, str: &String) -> &Self;
}
impl StringX for String {
	//TODO deprecate
	fn to_str(&self) -> &str { self.as_str() }
	fn push_string(&mut self, str: &String) -> &Self {
		self.push_str(str.to_str());
		self
	}
	fn push_str_line(&mut self, str: &str) -> &Self {
		self.push_str(str);
		self.push('\n');
		self
	}
}
