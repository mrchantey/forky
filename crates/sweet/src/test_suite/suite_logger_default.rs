use crate::*;
use forky_core::*;

#[derive(Default, Debug, Clone)]
pub struct SuiteLoggerDefault;


impl SuiteLogger for SuiteLoggerDefault {
	fn on_start(_: String) -> Self { Self }
	fn on_end(self, end_str: String) {
		log!("{end_str}");
	}
}
