use crate::*;
use forky_core::*;

#[derive(Default, Debug, Clone)]
pub struct SuiteLoggerNoop;


impl SuiteLogger for SuiteLoggerNoop {
	fn on_start(_: String) -> Self { Self }
	fn on_end(self, end_str: String) {
		log!("{end_str}");
	}
}
