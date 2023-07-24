use crate::TestSuiteResult;
use colorize::*;

pub trait SuiteLogger
where
	Self: Sized,
{
	fn log(val: &str);
	fn append_log(&mut self, val: &str);
	fn on_start() -> Self;
	fn on_end(self, running_indicator: bool);

	fn start(file: &str, running_indicator: bool) -> Self {
		if running_indicator {
			Self::log_start(file);
		}
		Self::on_start()
	}

	fn log_start(file: &str) {
		if cfg!(target_arch = "wasm32") {
			return;
		}
		let mut prefix = " RUNS ".black().bold().yellowb();
		prefix += " ";
		prefix += pretty_path(file).as_str();
		Self::log(prefix.as_str());
	}


	fn end(
		self,
		file: &str,
		running_indicator: bool,
		result: &TestSuiteResult,
	) {
		self.log_end(file, result);
		self.on_end(running_indicator);
	}

	fn log_end(&self, file: &str, result: &TestSuiteResult) {
		let mut prefix = if result.failed == 0 {
			" PASS ".black().bold().greenb()
		} else {
			" FAIL ".black().bold().redb()
		};
		prefix += " ";
		prefix += pretty_path(file).as_str();
		Self::log(prefix.as_str());
	}
}

fn pretty_path(val: &str) -> String {
	let mut splt = val.split("\\").collect::<Vec<&str>>();
	let _file = splt.pop();
	let file = _file.unwrap_or_default().to_string().bold();
	let path = splt.join("\\").faint();
	let middle = "\\".to_string().faint();
	format!("{path}{middle}{file}")
}

// fn get_now() -> Duration {
// 	#[cfg(target_arch = "wasm32")]
// 	return Duration::from_millis(
// 		web_sys::window().unwrap().performance().unwrap().now() as u64,
// 	);
// 	#[cfg(not(target_arch = "wasm32"))]
// 	return std::time::Instant::now().elapsed(); //TODO now().elapsed is incorrect
// }
