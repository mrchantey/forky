use crate::*;
use forky_core::*;
// use forky_web::LogBuffer;

pub struct SuiteLoggerWasm {
	pub log: String,
	// pub buffer_log: LogBuffer,
	// pub buffer_err: LogBuffer,
}


impl SuiteLogger for SuiteLoggerWasm {
	fn log(val: &str) {
		log!("{val}");
	}
	fn get_log(&mut self) -> &mut String { &mut self.log }

	fn on_start() -> Self {
		SuiteLoggerWasm {
			log: String::from("\n"),
			// buffer_log: LogBuffer::new_log(),
		}
	}

	fn on_end(self, _running_indicator: bool) {
		// self.log.push_str(self.buffer_log.end().as_str());
		log!("{}", self.log);
	}
}
