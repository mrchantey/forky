use crate::*;
use forky_core::*;
// use forky_web::LogBuffer;

pub struct SuiteLoggerWasm {
	pub logger_export: LoggerExport,
}
// pub log: String,
// pub buffer_log: LogBuffer,
// pub buffer_err: LogBuffer,


//TODO in progress erasable
impl SuiteLogger for SuiteLoggerWasm {
	fn on_start(_: String) -> Self {
		let logger_export = LoggerExport::get_or_init();
		Self { logger_export }
	}

	fn on_end(self, end_str: String) {
		log!("{end_str}");
		self.logger_export.push(&end_str);
	}
}
