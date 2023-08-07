use crate::*;
use forky_core::*;

#[derive(Default, Debug, Clone)]
pub struct SuiteLoggerNoop {
	pub log: String,
}


impl SuiteLogger for SuiteLoggerNoop {
	fn log(val: &str) {
		log!("{val}");
	}
	fn get_log(&mut self) -> &mut String { &mut self.log }

	fn on_start() -> Self {
		Self {
			log: String::from("\n"),
		}
	}

	fn on_end(self, _running_indicator: bool) {
		log!("{}", self.log);
	}
}
