use forky_core::testing::*;
mod misc;
mod test_runner;

fn main() -> Result<(), MatcherError> {
	TestRunner::run()?;
	Ok(())
}
