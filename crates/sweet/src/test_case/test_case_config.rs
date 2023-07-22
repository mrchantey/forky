
#[derive(Debug, Clone, PartialEq, Default)]
pub enum TestCaseConfig {
	#[default]
	Default,
	Skip,
	Only,
}
impl TestCaseConfig {
	pub fn to_i32(&self) -> i32 {
		match self {
			Self::Default => 0,
			Self::Skip => 1,
			Self::Only => 2,
		}
	}
	pub fn from_i32(val: i32) -> Self {
		match val {
			0 => Self::Default,
			1 => Self::Skip,
			2 => Self::Only,
			_ => Self::Default,
		}
	}
}
