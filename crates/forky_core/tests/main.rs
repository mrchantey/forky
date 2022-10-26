use forky_core::testing::*;
mod misc;
mod test_runner;
// use misc::*;

fn main() -> Result<(), MatcherError> {
	run()?;
	Ok(())
}
