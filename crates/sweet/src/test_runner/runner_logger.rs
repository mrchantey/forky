use crate::*;

pub trait RunnerLogger
where
	Self: Sized,
{
	fn start(config: &TestRunnerConfig) -> Self;
	fn end(self, results: &TestRunnerResult);

	fn pretty_print_intro(config: &TestRunnerConfig) -> String {
		format!("\n🤘 sweet as! 🤘\n\n{config}")
	}

}
