use forky_test::*;
mod test_runner;

fn main() -> Result<(), MatcherError> {
	TestRunner::run()
}
