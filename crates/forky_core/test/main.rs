use forky_test::*;
mod extensions;
mod misc;

fn main() -> Result<(), MatcherError> {
	TestRunner::run()?;
	Ok(())
}
