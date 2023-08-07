// use crate::*;
// use std::io::stdout;
// use std::io::Write;


// #[derive(Default, Debug, Clone)]
// pub struct SuiteLoggerNativeParallel;


// impl SuiteLogger for SuiteLoggerNativeParallel {

// 	fn on_start(_: String) -> Self {
// 		Self {
// 			cache: String::from("\n"),
// 		}
// 	}

// 	fn on_end(self, end_str: String) {
// 		println!("{end_str}");
// 	}
// }
