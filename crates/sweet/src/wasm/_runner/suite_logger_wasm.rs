use crate::*;
use forky_core::*;
// use forky_web::LogBuffer;

pub struct SuiteLoggerWasm;
// pub log: String,
// pub buffer_log: LogBuffer,
// pub buffer_err: LogBuffer,


//TODO in progress erasable
impl SuiteLogger for SuiteLoggerWasm {
	fn on_start(_: String) -> Self { SuiteLoggerWasm }

	fn on_end(self, end_str: String) {
		log!("{end_str}");
	}
}
