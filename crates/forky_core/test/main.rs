use forky_test::*;
mod extensions;
mod math;
mod misc;

fn main() -> Result<(), MatcherError> {
	test_runner::run()?;
	
	Ok(())
}
