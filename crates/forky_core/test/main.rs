use forky_test::*;
mod extensions;
mod math;
mod misc;

fn main() -> Result<(), MatcherError> {
	TestRunner::run()?;
	Ok(())
}
