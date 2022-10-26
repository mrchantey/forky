use std::fmt;

#[derive(Debug, Clone)]
pub struct MatcherError {
	pub message: String,
}
impl fmt::Display for MatcherError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}",self.message)
	}
}

pub type MatcherResult = Result<(), MatcherError>;
